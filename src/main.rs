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
pub extern "C" fn is_os() -> bool { false }
#[no_mangle]
pub extern "C" fn os_name() -> &'static str { "none" }
#[no_mangle]
pub extern "C" fn is_initramfs() -> bool { false }
#[no_mangle]
pub extern "C" fn device_name() -> &'static str { "default" }
#[no_mangle]
pub unsafe extern "C" fn initramfs_main() { return; }
#[no_mangle]
pub extern "C" fn kernfs_name() -> &'static str { "kernel filesystem" }
#[no_mangle]
pub unsafe extern "C" fn kernfs_init() { return; }

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    draw(Color::Cyan);
    die()
}
