#![no_std]

#[macro_use] extern crate novuskinc;

static mut SUM: i32 = 0;

pub fn ex1_init() {
    unsafe { SUM += 1; }
}


pub fn ex1_end() {
    unsafe { printk!("SUM = {}\n", SUM); }
}
