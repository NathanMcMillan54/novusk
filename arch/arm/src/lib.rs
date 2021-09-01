#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler, alloc_layout_extra, panic_info_message)]
#![feature(stmt_expr_attributes)]

#[macro_use] extern crate alloc;
#[cfg(target_arch = "arm")]
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate kinfo;

#[cfg(target_arch = "arm")]
pub mod boot;
#[cfg(target_arch = "arm")]
pub mod kernel;

pub mod include;
pub mod mm;

#[cfg(target_arch = "aarch64")]
#[cfg(feature = "rpi3")]
pub mod rpi;

#[cfg(feature = "nrf")]
pub mod nrf;
