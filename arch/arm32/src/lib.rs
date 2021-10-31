#![no_std]
#![feature(alloc_error_handler, panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

#[cfg(feature = "cortex_m")]
#[macro_use] extern crate cortex_m_semihosting;

#[cfg(feature = "stellaris_6965")]
pub(crate) extern crate stellarisd;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod mm;
pub mod net;

// CPUs
pub(crate) mod cortex_m;
