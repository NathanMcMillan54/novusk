#![no_std]

extern "C" { fn addInt(num1: u64, num2: u64) -> u64; }
static mut SUM: u64 = 0;

pub unsafe fn ex1_init() {
    SUM = addInt(1, 1);
}

pub unsafe fn ex1_exit() {
    let mut number = 0;
    number = SUM;
}
