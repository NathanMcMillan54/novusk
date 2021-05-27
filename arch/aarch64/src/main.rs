#![no_std]
#![no_main]

#[macro_use] extern crate aarch64_novusk;

#[no_mangle]
pub extern "C" fn application_type() -> libn::libnu::ktypes::ApplicationType { return libn::libnu::ktypes::ApplicationType::None; }

#[no_mangle]
pub extern "C" fn main_color() -> &'static str { return "cyan"; }

#[no_mangle]
pub extern "C" fn initramfs() -> bool { return false; }

#[no_mangle]
pub extern "C" fn initramfs_main() { return; }
