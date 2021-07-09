#![no_std]
#![feature(global_asm)]

global_asm!(include_str!("add.S"));

extern "C" {
    fn add(num1: u64, num2: u64) -> u64;
}

pub unsafe fn ex1_init() {
    let sum = add(1, 1);
}
