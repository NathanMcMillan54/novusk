#![no_std]

pub static mut SUM: u64 = 0;

extern "C" { fn addInt(num1: u64, num2: u64) -> u64; }

pub unsafe fn ex1_init() {
    SUM = addInt(1, 2);
}

pub unsafe fn ex1_end() {
    let result = SUM;
}

pub const MODULE: &str = "ex1";
pub const AUTHOR: &str = "Nathan McMillan <nathanmcmillan54@gmail.com>";