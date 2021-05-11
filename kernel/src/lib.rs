#![no_std]

#[macro_use] extern crate log;

pub mod info;

extern "C" {
    pub(crate) fn printk_init() -> bool;
}
