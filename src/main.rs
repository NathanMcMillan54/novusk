#![no_std]
#![no_main]

#[macro_use] extern crate alloc;
#[macro_use] pub extern crate novusk;

use pc_keyboard::PcKeyboard;
use alloc::string::String;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    printk::printk!("\nKernel Main");

    let mut keyboard = PcKeyboard::new();
    let bytes = keyboard.read_bytes();
    let mut string = String::new();

    printk::printk!("bytes have been read");

    printk::printk!("input: {:?}", string);

    loop {  }
}

#[no_mangle]
pub extern "C" fn initramfs_main() {

}
