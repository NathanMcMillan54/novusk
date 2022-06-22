#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]

#[macro_use] extern crate novuskinc;

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_semihosting::hprintln;
use hio::{HioDriver};
use hio::io::HioWriter;
use novuskinc::drivers::{Driver, manager::DeviceDriverManager, names::CONSOLE};
use novuskinc::kernel::{types::KernelFunctionName};

#[path = "dif.rs"]
pub mod dif;

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
}

pub(crate) mod common;
pub(crate) mod s6965;
pub(crate) mod s811;

fn stellaris_init() -> u8 {
    common::stellaris_board_init();

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, stellaris_init);

unsafe fn early_stellaris_init() -> u8 {
    DEVICE_DRIVERS.add_driver(&HioDriver as &'static dyn Driver);

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_stellaris_init);
