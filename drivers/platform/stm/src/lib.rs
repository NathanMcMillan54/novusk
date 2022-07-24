#![no_std]
#![feature(asm)]
#![feature(panic_info_message)]

#[macro_use] extern crate asminc;
extern crate invic;
#[macro_use] extern crate novuskinc;

use novuskinc::kernel::types::KernelFunctionName;

#[path = "dif.rs"]
mod dif;

pub mod clocks;
pub mod gpio;
pub mod irqs;

unsafe fn stm_init() -> u8 {
    clocks::setup_clocks();
    gpio::gpio_init();

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, stm_init);

unsafe fn stm_early_init() -> u8 {


    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, stm_early_init);

#[no_mangle]
pub extern "C" fn early_serial_init() { }

#[no_mangle]
pub extern "C" fn device_indicate_panic() { }

#[no_mangle]
pub extern "C" fn handle_irq() { }
