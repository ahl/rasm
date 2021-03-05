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

    extern "C" {
        #[cfg_attr(
            target_os = "macos",
            link_name = "\x01section$start$__DATA$__dtrace_probes"
        )]
        #[cfg_attr(target_os = "illumos", link_name = "__start_set_linkme_dtrace_probes")]
        static dtrace_probes_start: usize;
        #[cfg_attr(
            target_os = "macos",
            link_name = "\x01section$end$__DATA$__dtrace_probes"
        )]
        #[cfg_attr(target_os = "illumos", link_name = "__stop_set_linkme_dtrace_probes")]
        static dtrace_probes_stop: usize;
    }

    let data = unsafe {
        let start = (&dtrace_probes_start as *const usize) as usize;
        let stop = (&dtrace_probes_stop as *const usize) as usize;

        println!("{:#x} {:#x}", start, stop);

        std::slice::from_raw_parts(start as *const u8, stop - start)
    };

    println!("{:?}", data.hex_dump());
    println!("linkme {:?}", (&dtrace_probes as &[u8]).hex_dump());

    println!("done");
}
