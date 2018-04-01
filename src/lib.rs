#![no_std]
#![feature(asm, lang_items, core_intrinsics)]


pub mod entry_point;
pub mod syscall;

pub mod prelude {
    use syscall::exit;

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
}
