#![no_std]
#![no_main]

#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate cortex_m_semihosting;

use core::panic::PanicInfo;

pub mod boot;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    hprintln!("Panic!");

    loop {  }
}
