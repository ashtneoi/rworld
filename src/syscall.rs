use core::intrinsics;

// TODO: understand cfg
#[cfg(target_arch = "x86_64")]
pub unsafe fn syscall0(nr: usize) -> isize {
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
pub unsafe fn syscall1(nr: usize, x1: usize) -> isize {
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
