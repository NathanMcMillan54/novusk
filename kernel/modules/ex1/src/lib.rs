#![no_std]

extern "C" {
    fn add(num1: i32, num2: i32) -> i32;
}

pub unsafe fn ex1_init() {
    let sum = add(1, 1);
}
