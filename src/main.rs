#![no_std]
#![no_main]

#[macro_use]
extern crate novusk;
use include::novusk::syscalls::draw;
use kernel::die;
use libn::color::vga::Color;

#[no_mangle]
pub unsafe extern "C" fn main_test() -> i32 {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    draw(Color::Cyan);
    die()
}
