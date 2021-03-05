#[macro_export]
macro_rules! _sdt_asm {
    () => {
        unsafe {
            let x: u64 = 0x1234_1234_1234_1234;
            asm!(
                r#"
                990:    nop

                        .section __DATA,__dtrace_probes,regular,no_dead_strip
                        .balign 8
                991:
                        .long 992f-991b     // length
                        .quad 990b          // offset
                        .quad {main}        // function
                        .asciz "provider"   // provider
                        .asciz "function"   // function
                        .asciz "probe"      // probe
                992:    .balign 8

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
