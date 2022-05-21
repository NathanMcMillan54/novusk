#![no_std]
#![no_main]

#[macro_use] extern crate cortex_m_rt;

use core::panic::PanicInfo;

mod boot;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {  }
}
