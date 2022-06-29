#![no_std]
#![no_main]

#[macro_use] extern crate cortex_m_rt;
pub(crate) extern crate stellaris;
pub(crate) extern crate nmallocator;

use core::panic::PanicInfo;

pub mod boot;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    // hprintln!("Panic!");

    loop {  }
}

#[no_mangle]
pub static KERNEL_NAME: &'static str = "Sellaris Novusk kernel";
