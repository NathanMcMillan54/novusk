#![no_std]
#![feature(global_asm)]

extern crate arm;

pub const ARCH: &'static str = "aarch64";

pub mod aarch64lib;
pub mod boot;
pub mod drivers;
pub mod kernel;

// include/src/time.rs
fn sleep(seconds: i32) {
    let mut x = 0;
    let time = seconds * 100000000;
    loop {
        x = x + 1;
        if x == time {
            break;
        }
    }
}
