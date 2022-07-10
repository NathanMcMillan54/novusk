#![no_std]
#![allow(warnings)]
#![feature(asm)]
#![feature(panic_info_message)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

pub mod display;
pub mod vga;

use libcolor::{Color16, ColorCode};
use novuskinc::drivers::Driver;
use novuskinc::drivers::{manager::DeviceDriverManager, names::CONSOLE};
use novuskinc::fb::*;
use vga::vga_80x25::{Vga80x25Buffer, Vga80x25};
use vga::{VgaG, VgaMode};

#[cfg(not(feature = "no_panic"))]
#[path = "../../../../kernel/panic.rs"]
pub mod panic;

#[no_mangle]
pub static mut FB: FrameBuffer = FrameBuffer::empty();

extern "C" {
    static mut DEVICE_DRIVERS: DeviceDriverManager;
}

unsafe fn vgag_init() {
    DEVICE_DRIVERS.add_driver(&VgaG as &dyn Driver);
}

module_init!(core_display_init, vgag_init);

unsafe fn vgag_end() {
    let vgag_driver = DEVICE_DRIVERS.get_driver(CONSOLE);

    if vgag_driver.is_none() {
        panic!("core_display wasn't initialized or was initialized unsuccessfully")
    } else if vgag_driver.unwrap().driver_name() != vga::VGAG_NAME {
        panic!("'Console Driver' in 'DEVICE_DRIVERS' is not the VGAG console driver");
    }

    vgag_driver.unwrap().init();
}

module_end!(core_display_end, vgag_end);
