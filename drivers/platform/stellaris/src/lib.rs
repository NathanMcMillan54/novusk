#![no_std]

#[macro_use] extern crate novuskinc;

fn stellaris_init() {

}


fn stellaris_end() {

}

#[no_mangle]
static mut DIF_FILE: &'static str = include_str!("../../../../arch/arm/src/include/dif/difs/stellaris6965.dif");

module_init!(early_device_init, stellaris_init);
module_end!(early_device_end, stellaris_end);

