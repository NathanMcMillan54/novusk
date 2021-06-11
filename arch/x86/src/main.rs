#![no_std]
#![no_main]

extern crate libnu;
use libnu::ktypes::ApplicationType;
#[macro_use] extern crate x86_novusk;
use x86_novusk::printk::printk;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    printk!("Kernel Main!");
}

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
pub extern "C" fn main_color() -> &'static str { return "green" }
