#![no_std]
#![feature(global_asm, llvm_asm)]

#[macro_use] extern crate log;
#[macro_use] extern crate uefi;
#[macro_use] extern crate uefi_macros;
#[macro_use] extern crate uefi_services;

// Include
extern crate ctypes;
extern crate novusk;

// Kernel
#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;
pub use printk::printk;

extern crate userspace;

#[cfg(target_arch = "x86")]
extern crate i686;

use ctypes::c_char;

pub(crate) mod boot;
pub mod drivers;
pub mod include;
pub mod kernel;

pub const TEST: *const c_char = "test".as_ptr() as *const i8;
