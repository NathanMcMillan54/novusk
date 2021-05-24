#![no_std]
#![no_main]

extern crate libnu;
use libnu::ktypes::ApplicationType;
extern crate x86_novusk;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! { loop {  } }

#[no_mangle]
pub extern "C" fn kernel_info() -> bool { return true; }

#[no_mangle]
pub extern "C" fn application_type() -> ApplicationType { return ApplicationType::None; }

#[no_mangle]
pub extern "C" fn initramfs() -> bool { return false; }

#[no_mangle]
pub extern "C" fn initramfs_main() { return; }

// TODO: Make a color type
#[no_mangle]
pub extern "C" fn main_color() -> &'static str { return "cyan" }
