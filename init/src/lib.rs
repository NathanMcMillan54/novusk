#![no_std]

#[macro_use] extern crate printk;

use core::panic::PanicInfo;

mod kmain;
pub mod version;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {  }
}
