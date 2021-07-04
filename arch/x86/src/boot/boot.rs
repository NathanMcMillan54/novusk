use super::bootloaders::bootloader_init;
use core::fmt::Arguments;
use crate::drivers::ix86::init::ix86_init;
use crate::drivers::vga::{VGA_ADDRESS, init::vga_init};
use crate::include::asm::hlt;
use crate::kernel::kernel::*;

#[cfg(feature = "bios_boot")]
pub const BOOT: &'static str = "BIOS";

#[cfg(feature = "uefi_boot")]
pub const BOOT: &'static str = "UEFI";


pub unsafe fn die() -> ! {
    panic!("Kernel died");
    loop { hlt(); }
}

#[no_mangle]
pub unsafe extern "C" fn boot_init() {
    if BOOT == "BIOS" {
        bios_setup();
    } else if BOOT == "UEFI" {
        uefi_setup();
    } else {  }
}

unsafe fn bios_setup() {
    #[cfg(not(feature = "vga_0xa"))]
    vga_init(80, 25, 0xb8000);

    #[cfg(feature = "vga_0xa")]
    vga_init(320, 200, 0xa0000);

    #[cfg(target_arch = "x86")]
    ix86_init();

    bootloader_init();
}

unsafe fn uefi_setup() {

}
