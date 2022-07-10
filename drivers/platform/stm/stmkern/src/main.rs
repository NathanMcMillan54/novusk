#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate arm;
extern crate asminc;
#[macro_use] extern crate cortex_m_rt;
extern crate stm;

use core::panic::PanicInfo;

mod boot;

#[no_mangle]
pub static KERNEL_NAME: &'static str = "STM Novusk";
