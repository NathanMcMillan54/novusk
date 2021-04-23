#![no_std]

#[macro_use]
extern crate kernel;
use kernel::info::{*};

extern "C" {
    fn is_os() -> bool;
    fn os_name() -> &'static str;
    fn is_initramfs() -> bool;
    fn device_name() -> &'static str;
}

pub unsafe fn m1_init() {
    IS_OS = is_os();
    OS_NAME = os_name();
    IS_INTRAMFS = is_initramfs();
    DEVICE_NAME = device_name();
}

pub unsafe fn m1_exit() {

}

pub const MODULE: &str = "m1";
pub const AUTHOR: &str = "Nathan McMillan";
