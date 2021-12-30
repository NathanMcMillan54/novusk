#![no_std]

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

#[no_mangle]
pub extern "C" fn kmain_printk(fmt: core::fmt::Arguments) {

}
