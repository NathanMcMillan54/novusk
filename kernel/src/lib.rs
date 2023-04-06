#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;

use novuskinc::kernel::Kernel;
use novuskinc::kernel::types::KernelType;
use spin::Mutex;
use alloc::vec;

pub mod kdif;

extern "C" {
    fn _kernel_version() -> (u8, u8, u8);
    fn _kernel_type() -> KernelType;
    fn _kernel_short() -> bool;
}

lazy_static! {
    pub static ref KERNEL: Mutex<Kernel> = Mutex::new(Kernel {
        version: unsafe { _kernel_version() },
        kernel_type: unsafe { _kernel_type() },
        short: unsafe { _kernel_short() },
        module_names: vec![],

    });
}