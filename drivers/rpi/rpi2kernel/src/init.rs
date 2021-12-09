use konfig::Konfig;
use crate::drivers::*;

#[no_mangle]
pub unsafe extern "C" fn rpi2_kernel_init() {
    fb::fb_init();
    kinfo!("Frame buffer an console initialized\n");

    panic!("RPi 2 kernel finished");
}
