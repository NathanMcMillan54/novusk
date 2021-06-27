use super::bootloaders::bootloader_init;
use core::fmt::Arguments;
use nkuefi::init::uefi_init;
use crate::drivers::ix86::init::ix86_init;
use crate::drivers::vga::{VGA_ADDRESS, init::vga_init};
use crate::include::asm::hlt;
use crate::kernel::kernel::*;

extern "C" { pub fn boot_method() -> &'static str; }

pub static mut BOOT: &'static str = "";

pub unsafe fn die() -> ! {
    panic!("Kernel died");
    loop { hlt(); }
}

#[no_mangle]
pub unsafe extern "C" fn boot_init() {
    BOOT = boot_method();
    if BOOT == "BIOS" {
        bios_setup();
    } else if BOOT == "UEFI" {
        uefi_setup();
    } else {  }
}

unsafe fn bios_setup() {
    vga_init(25, 80, 0xb8000);

    #[cfg(target_arch = "x86")]
    ix86_init();

    bootloader_init();
}

unsafe fn uefi_setup() {
    uefi_init();
}
