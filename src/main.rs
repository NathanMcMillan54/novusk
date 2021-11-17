#![no_std]
#![no_main]

#[macro_use] pub extern crate novusk;

use kb_mouse::input::kb_mouse_input;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    printk::printk!("\nKernel Main\n");

    loop {
        let (mouse_x, mouse_y) = kb_mouse_input();

        printk::printk!("X increase: {} Y increase: {}", mouse_x, mouse_y);
    }
}

#[no_mangle]
pub extern "C" fn initramfs_main() {

}
