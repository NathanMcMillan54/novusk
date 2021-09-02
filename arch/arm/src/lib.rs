#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod mm;

// CPUs
pub mod cortex_m3;
pub mod cortex_m4;