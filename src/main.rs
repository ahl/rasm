#![feature(asm)]
#![feature(decl_macro)]

use pretty_hex::PrettyHex;
use std::usize;

macro_rules! _sdt_asm {
    () => {
        unsafe {
            let x: u64 = 0x1234_1234_1234_1234;
            asm!(
                r#"


                990:    nop

                // Put some data into our secret __TEXT.__dtrace section.
                        .section __TEXT,__dtrace,regular,no_dead_strip
                991:
                        .long 992f-991b     // length
                        .quad 990b          // offset
                        .quad {main}        // function
                        .asciz "provider"   // provider
                        .asciz "function"   // function
                        .asciz "probe"      // probe
                992:    .balign 4

                // Only set _.dtrace.base the first time
                .ifndef _.dtrace.base
                        .set  _.dtrace.base, 991b
                .endif
                // Reset _.dtrace.size each time we encounter a probe
                        .set _.dtrace.size, 992b

                // Get back to the text section.
                .text
            "#,
            main = sym main,
            in("rdi") x,
            options(readonly, nostack, preserves_flags),
            )
        }
    };
}

fn foo() {
    _sdt_asm!();
}

mod foo {
    pub macro bar() {}
}

fn main() {
    println!("Hello, world!");

    foo::bar!();

    let x = unsafe {
        let x: u64;
        asm!("lea {0}, [rip+0]", out(reg) x);
        x
    };

    println!("%rip = {:#x}", x);

    _sdt_asm!();
    _sdt_asm!();

    foo();

    extern "C" {
        #[link_name = ".dtrace.base"]
        static dtrace_base: usize;
        #[link_name = ".dtrace.size"]
        static dtrace_size: usize;
    }

    let data = unsafe {
        let base = (&dtrace_base as *const usize) as usize;
        let size = (&dtrace_size as *const usize) as usize;

        println!("{:#x} {:#x}", base, size);

        std::slice::from_raw_parts(base as *const u8, size - base)
    };

    println!("{:?}", data.hex_dump());

    println!("done");
}
