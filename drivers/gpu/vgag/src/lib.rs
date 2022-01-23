#![no_std]
#![allow(warnings)]

#[macro_use] extern crate novuskinc;

fn vgag_init() {

}

module_init!(core_display_init, vgag_init);

fn vgag_end() {

}

module_end!(core_display_end, vgag_end);
