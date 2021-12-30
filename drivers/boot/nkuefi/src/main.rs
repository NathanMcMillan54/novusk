#![no_std]
#![no_main]

use uefi::Handle;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, st: SystemTable<Boot>) -> ! {

    panic!("Bootloader ended");
}
