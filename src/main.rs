#![feature(asm)]
#![feature(decl_macro)]

use pretty_hex::PrettyHex;
use std::usize;

#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "illumos", path = "illumos.rs")]
mod os;

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
        #[link_name = "__start_set_dtrace_base"]
        static dtrace_base: usize;
        #[link_name = "__stop_set_dtrace_base"]
        static dtrace_end: usize;


        //#[link_name = "__start_set_dtrace_base"]
        //static st: usize;
    }

    let data = unsafe {
        let base = (&dtrace_base as *const usize) as usize;
        let size = (&dtrace_end as *const usize) as usize;
        //let s = (&st as *const usize) as usize;
        //println!("s {:#x}", s);

        println!("{:#x} {:#x}", base, size);

        std::slice::from_raw_parts(base as *const u8, size - base)
    };

    println!("{:?}", data.hex_dump());

    println!("done");
}
