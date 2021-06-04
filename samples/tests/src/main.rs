#![no_std]
#![no_main]

pub mod required;

#[macro_use] extern crate x86_novusk;
use x86_novusk::ps2_keyboard::test::test_keyboard;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    test_keyboard();
    loop {  }
}
