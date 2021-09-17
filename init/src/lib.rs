#![no_std]
#![feature(const_fmt_arguments_new)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

pub mod init;
pub mod initramfs;
pub mod kmain;
pub mod noinitramfs;
pub mod modules;
pub mod version;
