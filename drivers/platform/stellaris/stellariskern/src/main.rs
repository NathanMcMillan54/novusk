#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use] extern crate cortex_m_rt;
pub extern crate irq;
pub(crate) extern crate nmallocator;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;
pub(crate) extern crate stellaris;

use asminc::arm32::wfi;
use core::panic::PanicInfo;
use cortex_m::peripheral::NVIC;
use novuskinc::irq::IrqHandler;

pub mod boot;
pub mod kernel;

#[no_mangle]
pub static KERNEL_NAME: &'static str = "Sellaris Novusk";
