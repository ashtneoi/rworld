#![no_std]
#![feature(start)]
extern crate rworld;

use rworld::prelude::*;
use rworld::syscall;

#[start]
fn cmain(argc: isize, argv: *const *const u8) -> isize {
    go_rust(argc, argv, main);
}

fn main(argc: isize, _argv: *const *const u8, _envp: *const *const u8) {
    syscall::exit(argc);
}
