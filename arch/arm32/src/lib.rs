#![no_std]
#![feature(alloc_error_handler)]

#[macro_use] extern crate lazy_static;

#[cfg(feature = "stellaris_6965")]
pub(crate) extern crate stellarisd;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod mm;

// CPUs
pub mod cortex_m;
