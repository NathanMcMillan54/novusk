#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]

#[macro_use] extern crate cortex_m_rt;
pub extern crate invic;
#[macro_use] extern crate novuskinc;

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use hio::{HioDriver};
use hio::io::HioWriter;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, manager::DeviceDriverManager, names::CONSOLE};
use novuskinc::kernel::{types::KernelFunctionName};

#[path = "dif.rs"]
pub mod dif;

extern "C" {
    pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager;
    pub(crate) static mut DIF: ::dif::Dif;
}

pub mod clocks;
pub(crate) mod common;

#[cfg(feature = "s6965")]
pub(crate) mod s6965;

#[cfg(feature = "s811")]
pub(crate) mod s811;

fn stellaris_init() -> u8 {
    common::stellaris_board_init();

    #[cfg(feature = "s6965")]
    s6965::lm3s6965_init();

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, stellaris_init);

/// Sets drivers for a Stellaris board to ``DEVICE_DRIVERS`` so they can be initialized later
unsafe fn early_stellaris_init() -> u8 {
    if DIF.get("PrintingMethod").1 == "Hio" {
        DEVICE_DRIVERS.add_driver(&HioDriver as &'static dyn Driver);
    }

    if DEVICE_DRIVERS.get_driver("Console Driver").is_none() {
        return 1;
    }

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_stellaris_init);
