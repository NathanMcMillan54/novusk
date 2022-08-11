#![no_std]
#![feature(concat_idents)]

#[macro_use] extern crate alloc;

pub mod console;
pub mod defs;
pub mod elf;
pub mod fb;
pub mod fs;
pub mod firmware;
pub mod drivers;
pub mod gpu;
pub mod input;
pub mod irq;
pub mod keyboard;
pub mod kernel;
pub mod led;
pub mod mb;
pub mod module;
pub mod net;
pub mod platform;
pub mod power;
pub mod prelude;
pub mod serial;
pub mod storage;
pub mod timer;
pub mod version;
