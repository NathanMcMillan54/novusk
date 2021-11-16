#![no_std]

#[macro_use] extern crate novuskinc;

static mut SUM: i32 = 0;

pub fn ex1_start() {
    unsafe { SUM += 1; }
}

module_init!(ex1_init, ex1_start);

pub fn ex1_finish() {
    unsafe { printk!("SUM = {}\n", SUM); }
}

module_end!(ex1_end, ex1_finish);
