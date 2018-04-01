#![no_std]
#![feature(linkage, start)]
extern crate rworld;

use rworld::{go_rust, exit};

#[start]
fn cmain(argc: isize, argv: *const *const u8) -> isize {
    go_rust(argc, argv, main);
}

fn main(argc: isize, _argv: *const *const u8, _env: *const *const u8) {
    exit(argc);
}
