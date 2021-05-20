#![no_std]

static mut SUM: i32 = 0;

pub unsafe fn armm_init() {
    SUM = 1 + 1;
}

pub unsafe fn armm_end() {
    return;
}
