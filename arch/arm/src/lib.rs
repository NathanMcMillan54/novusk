#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler, alloc_layout_extra)]
#![feature(stmt_expr_attributes)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate kinfo;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod mm;

#[cfg(feature = "rpi")]
pub mod rpi;

#[cfg(feature = "nrf")]
pub mod nrf;
