#![no_std]
#![no_main]

extern crate novusk;
use novusk::die;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    die()
}
