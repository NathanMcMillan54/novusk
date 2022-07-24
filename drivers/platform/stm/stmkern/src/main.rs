#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate arm;
extern crate asminc;
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate novuskinc;
extern crate stm;

mod boot;
pub mod kernel;

#[no_mangle]
pub static KERNEL_NAME: &'static str = "STM Novusk";
