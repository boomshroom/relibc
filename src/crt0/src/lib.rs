//! crt0

#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![feature(naked_functions)]

extern crate platform;

use platform::types::*;

#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() {
    #[cfg(target_arch = "x86_64")]
    asm!("mov rdi, rsp
        call _start_rust"
        :
        :
        :
        : "intel", "volatile"
    );
    #[cfg(target_arch = "aarch64")]
    asm!("mov x0, sp
        bl _start_rust"
        :
        :
        :
        : "volatile"
    );
}

#[repr(C)]
pub struct Stack {
    argc: isize,
    argv0: *const u8,
}

impl Stack {
    fn argc(&self) -> isize {
        self.argc
    }

    fn argv(&self) -> *const *const u8 {
        &self.argv0 as *const *const u8
    }
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn _start_rust(sp: &'static Stack) -> ! {
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> c_int;
        fn _init();
        fn _fini();
    }

    _init();

    let argc = sp.argc();
    let argv = sp.argv();

    let code = main(argc, argv);

    _fini();

    platform::exit(code);
}

#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_unwind(_fmt: ::core::fmt::Arguments, _file: &str, _line: u32) -> ! {
    loop {}
}
