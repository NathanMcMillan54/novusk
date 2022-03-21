#![no_std]

#[macro_use] extern crate novuskinc;

#[path = "dif.rs"]
mod dif;

fn stm_init() {

}

fn stm_end() {

}

module_init!(early_device_init, stm_init);
module_end!(early_device_end, stm_end);
