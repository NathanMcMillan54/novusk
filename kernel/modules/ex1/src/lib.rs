#![no_std]

static mut SUM: i32 = 0;

pub unsafe fn ex1_init() {
    SUM += 1;
    ex1_exit();
}

pub unsafe fn ex1_exit() {

}
