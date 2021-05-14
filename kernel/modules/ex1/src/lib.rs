#![no_std]
#![feature(global_asm)]

global_asm!(include_str!("add.S"));

extern crate novusk;

pub static mut SUM: u64 = 0;

pub unsafe fn ex1_init() {
    let add = addInt(1, 1);
    SUM = add;
}

pub unsafe fn ex1_exit() {
    return;
}

extern "C" { fn addInt(num1: u64, num2: u64) -> u64; }
