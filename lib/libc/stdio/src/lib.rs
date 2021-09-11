#![no_std]
#![feature(c_variadic)]

use ctypes::{c_char, c_int};

pub static mut TERMINAL_ROW: c_int = 1;

extern "C" {
    pub fn printf(fmt: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn ttprintf(fmt: *const char, ...) -> c_int {
    return 0;
}
