#![no_std]

#[macro_use] extern crate novuskinc;

// When the version can be determined in the future call one of these
pub(crate) mod blue;
pub(crate) mod cl;
pub(crate) mod yellow;

pub fn fom_os_init() {
    // In Novusk v4 (rewrite) everything will be compiled individually which will stop linker errors
    // like this:
    // unsafe { start_module!(fn_dev_init, fn_dev_end); }
}
