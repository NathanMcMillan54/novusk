#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate irq;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

#[cfg(feature = "nrf52840")]
pub extern crate nrfd;

#[cfg(feature = "stellaris_6965")]
pub extern crate stellarisd;

#[cfg(feature = "stm32f4")]
pub extern crate stmd;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod mm;

// CPUs
pub mod cortex_m3;
pub mod cortex_m4;
