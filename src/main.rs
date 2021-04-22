#![no_std]
#![no_main]

#[macro_use]
extern crate novusk;
use novusk::die;

#[no_mangle]
pub unsafe extern "C" fn main_test() -> i32 {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    die()
}
