#![no_std]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

use novuskinc::module::*;

static mut SUM: i32 = 0;

unsafe fn _init_ex1(km: &mut KernelModule) {
    unsafe { SUM += 1; }
    km.initialized();
}

module_init!(ModuleType::InKernel, ex1);

unsafe fn _end_ex1(km: &mut KernelModule) {
    if SUM == 1 {
        km.success();
    }
}

module_end!(ModuleType::InKernel, ex1);
