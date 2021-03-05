#![feature(asm)]
#![feature(decl_macro)]

use linkme::distributed_slice;
use pretty_hex::PrettyHex;

#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "illumos", path = "illumos.rs")]
mod os;

fn foo() {
    _sdt_asm!();
}

mod foo {
    pub macro bar() {}
}

#[allow(non_upper_case_globals)]
#[distributed_slice]
static dtrace_probes: [u8] = [..];

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

    /*
    extern "C" {
        #[cfg_attr(target_os = "macos", link_name = ".dtrace.base")]
        #[cfg_attr(target_os = "illumos", link_name = "__start_set_dtrace_base")]
        static dtrace_base: usize;
        #[cfg_attr(target_os = "macos", link_name = ".dtrace.end")]
        #[cfg_attr(target_os = "illumos", link_name = "__end_set_dtrace_base")]
        static dtrace_end: usize;
    }

    let data = unsafe {
        let base = (&dtrace_base as *const usize) as usize;
        let size = (&dtrace_end as *const usize) as usize;

        println!("{:#x} {:#x}", base, size);

        std::slice::from_raw_parts(base as *const u8, size - base)
    };

    println!("{:?}", data.hex_dump());
    */
    println!("{:?}", (&dtrace_probes as &[u8]).hex_dump());

    println!("done");
}
