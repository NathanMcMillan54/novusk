#![no_std]
#![feature(concat_idents)]

#[macro_use] extern crate cfg_if;

#[cfg(target_arch = "aarch64")]
#[macro_use] extern crate tock_registers;

pub(crate) extern crate vfs;

pub use printk::printk;
pub use kinfo::kinfo;

pub mod defs;
pub mod gpu;
pub mod input;
pub mod kernel;
pub mod led;
pub mod module;
pub mod power;
pub mod fs;
pub mod version;
