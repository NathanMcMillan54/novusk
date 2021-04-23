#![no_std]

extern crate include;

pub mod info;
pub mod kinfo;
pub mod printk;

extern "C" { pub fn die() -> !; }
