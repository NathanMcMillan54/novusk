#![no_std]
#![feature(panic_info_message)]

#[macro_use] extern crate printk;

use core::panic::PanicInfo;

mod kmain;
pub mod version;

#[path = "../../kernel/panic.rs"]
mod panic;
