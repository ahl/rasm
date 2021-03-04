#[macro_export]
macro_rules! _sdt_asm {
    () => {
        unsafe {
            let x: u64 = 0x1234_1234_1234_1234;
            asm!(
                r#"
                990:    nop

                // Put some data into our secret .dtrace.base section.
                        .pushsection .dtrace.base,"a","progbits"
                        .balign 8

                991:
                        .4byte 992f-991b     // length
                        .8byte 990b          // offset
                        .8byte {main}        // function
                        .asciz "provider"   // provider
                        .asciz "function"   // function
                        .asciz "probe"      // probe
                992:    .balign 8
                        .set .dtrace.end, 992b
                        .popsection
            "#,
            main = sym main,
            in("rdi") x,
            options(readonly, nostack, preserves_flags),
            )
        }
    };
}
