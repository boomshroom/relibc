//! crti

#![no_std]
#![feature(global_asm)]
#![feature(lang_items)]

#[cfg(target_arch = "x86_64")]
global_asm!(r#"
    .section .init,"ax",@progbits
    .global _init
    .type _init, @function
    _init:
        push %rbp
        movq %rsp, %rbp

    .section .fini,"ax",@progbits
    .global _fini
    .type _fini, @function
    _fini:
    	push %rbp
    	movq %rsp, %rbp
"#);

// Shamelessly stolen from musl
#[cfg(target_arch = "aarch64")]
global_asm!(r#"
    .section .init,"ax",@progbits
    .global _init
    .type _init, @function
    _init:
        stp x29,x30,[sp,-16]!
        mov x29,sp

    .section .fini,"ax",@progbits
    .global _fini
    .type _fini, @function
    _fini:
        stp x29,x30,[sp,-16]!
        mov x29,sp
"#);

#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_unwind(_fmt: ::core::fmt::Arguments, _file: &str, _line: u32) -> ! {
    loop {}
}
