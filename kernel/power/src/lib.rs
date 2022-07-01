#![no_std]

#[macro_use] extern crate novuskinc;

use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::power::*;

unsafe fn kernel_set_pm(mode: u32) -> u32 {
    match mode {
        PM_REBOOT => reboot(),
        PM_SHUTDOWN => shutdown(),
        _ => return 0,
    };

    0
}

define_kernel_function!(KernelFunctionName::set_power_mode, mode: u32, -> u32, kernel_set_pm);
