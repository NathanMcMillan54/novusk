#![no_std]
#![no_main]

#[macro_use] extern crate novusk;

use novuskinc::fb::{Color, FrameBufferGraphics};
use armfb::ArmFb;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    printk::printk!("\nKernel Main\n");
}

#[no_mangle]
pub extern "C" fn initramfs_main() {

}
