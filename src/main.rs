#![no_std]
#![no_main]

#[macro_use] extern crate novusk;

use unistd::syscall;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    printk::printk!("\nKernel Main\n");
}

#[no_mangle]
pub extern "C" fn initramfs_main() {

}
