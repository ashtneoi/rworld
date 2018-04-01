use core::intrinsics;
use syscall::exit;

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
    _: ::core::fmt::Arguments,
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
    exit(start(argc, argv));
}
