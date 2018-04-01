#![no_std]
#![feature(asm, lang_items, core_intrinsics, start)]
extern crate rlibc;

use core::intrinsics;

pub fn getpid() -> isize {
    unsafe {
        syscall0(39)
    }
}

pub fn exit(status: isize) -> ! {
    unsafe {
        syscall1(60, status as usize);
        intrinsics::abort()
    }
}

// TODO: understand cfg
#[cfg(target_arch = "x86_64")]
unsafe fn syscall0(nr: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall"
        : "={rax}"(ret)
        : "{rax}"(nr)
        : "rcx", "r11", "memory"
        : "volatile"
    );
    ret
}

#[cfg(target_arch = "x86_64")]
unsafe fn syscall1(nr: usize, x1: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall"
        : "={rax}"(ret)
        : "{rax}"(nr), "{rdi}"(x1)
        : "rcx", "r11", "memory"
        : "volatile"
    );
    ret
}

#[no_mangle]
pub extern fn __libc_csu_init() {
}

#[no_mangle]
pub extern fn __libc_csu_fini() {
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(
    _: core::fmt::Arguments,
    _: &'static str,
    _: u32,
    _: u32,
) -> ! {
    unsafe {
        intrinsics::abort()
    }
}

#[no_mangle]
pub extern fn __libc_start_main(
    start: fn(isize, *const *const u8) -> isize,
    argc: isize,
    argv: *const *const u8,
) {
    unsafe {
        let _ep = argv.offset(argc + 1);
    }
    exit(start(argc, argv));
}

pub fn go_rust(
    argc: isize,
    argv: *const *const u8,
    f: fn(isize, *const *const u8, *const *const u8),
) -> ! {
    let envp;
    unsafe {
        envp = argv.offset(argc + 1);
    }
    f(argc, argv, envp);
    exit(0);
}
