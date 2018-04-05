//! crtn

#![no_std]
#![feature(global_asm)]
#![feature(lang_items)]

// Shamelessly stolen from the OSDev wiki
#[cfg(target_arch = "x86_64")]
global_asm!(r#"
    .section .init,"ax",@progbits
        popq %rbp
        ret

    .section .fini,"ax",@progbits
        popq %rbp
        ret
"#);

// Shamelessly stolen from musl
#[cfg(target_arch = "aarch64")]
global_asm!(r#"
    .section .init,"ax",@progbits
        ldp x29,x30,[sp],#16
        ret

    .section .fini,"ax",@progbits
        ldp x29,x30,[sp],#16
        ret
"#);

#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_unwind(_fmt: ::core::fmt::Arguments, _file: &str, _line: u32) -> ! {
    loop {}
}
