#[macro_export]
macro_rules! _sdt_asm {
    () => {
        unsafe {
            let x: u64 = 0x1234_1234_1234_1234;
            asm!(
                r#"
                990:    nop

                // Put some data into our secret __TEXT.__dtrace section.
                        .section __TEXT,__dtrace,regular,no_dead_strip
                        .balign 8
                991:
                        .long 992f-991b     // length
                        .quad 990b          // offset
                        .quad {main}        // function
                        .asciz "provider"   // provider
                        .asciz "function"   // function
                        .asciz "probe"      // probe
                992:    .balign 8

                // Only set _.dtrace.base the first time
                .ifndef _.dtrace.base
                        .set  _.dtrace.base, 991b
                .endif
                // Reset _.dtrace.end each time we encounter a probe
                        .set _.dtrace.end, 992b

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
