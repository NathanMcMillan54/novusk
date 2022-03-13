#![no_std]

#[macro_use] extern crate novuskinc;

#[path = "dif.rs"]
pub mod dif;

fn stellaris_init() {

}


fn stellaris_end() {

}

module_init!(early_device_init, stellaris_init);
module_end!(early_device_end, stellaris_end);
