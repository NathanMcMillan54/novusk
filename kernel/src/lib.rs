#![no_std]

#[macro_use] extern crate log;

extern "C" {
    pub(crate) fn printk_init() -> bool;
}
