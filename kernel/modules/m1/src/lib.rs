#![no_std]

#[macro_use]
extern crate kernel;

pub unsafe fn m1_init() {
    // printk!("   M1 init\n");
}

pub unsafe fn m1_exit() {
    // printk!("   M1 end\n");
}

pub const MODULE: &str = "m1";
pub const AUTHOR: &str = "Nathan McMillan <nathanmcmillan54@gmail.com>";
