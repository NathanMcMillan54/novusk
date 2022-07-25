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
pub mod irqs;
pub mod panic;

/// STM32f4xx boards
#[cfg(feature = "stm32f4")]
pub mod f4;

#[cfg(feature = "stm32f4")]
pub use f4::board;

unsafe fn stm_init() -> u8 {
    board::stm_board_init();

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
pub extern "C" fn handle_irq() { }
