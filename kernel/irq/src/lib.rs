#![no_std]
#![feature(panic_info_message)]

#[macro_use] extern crate alloc;

#[cfg(feature = "staticlib")]
extern crate nmallocator;

#[cfg(feature = "staticlib")]
#[path = "../../panic.rs"]
pub mod panic;

pub mod chip;
pub mod handle;
