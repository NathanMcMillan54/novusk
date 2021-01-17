#![feature(asm, global_asm)]
#![no_std]

pub mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const ARCH: &'static str = "x86";

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
