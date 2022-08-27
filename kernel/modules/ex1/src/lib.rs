#![no_std]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

use novuskinc::module::*;

static mut SUM: i32 = 0;

pub fn _init_ex1() {
    unsafe { SUM += 1; }
}

module_init!(ModuleType::InKernel, ex1);

pub fn _end_ex1() {
    unsafe { printk!("SUM = {}\n", SUM); }
}

module_end!(ex1);
