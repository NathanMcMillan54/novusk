#![no_std]

static mut SUM: i32 = 0;

pub unsafe fn ex1_init() {
    SUM += 1;
}

pub unsafe fn ex1_exit() {
    assert_eq!(SUM, 1);
}
