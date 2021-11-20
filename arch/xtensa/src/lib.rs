#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate xtensa_lx_rt;

#[cfg(any(feature = "esp32"))]
pub(crate) extern crate esp as board;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod mm;