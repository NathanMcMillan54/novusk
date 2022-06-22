use bootloader::BootInfo;
use super::boot::{die, boot_init, BOOT};
use core::ptr::write_volatile;
use crate::kernel::kernel::*;
use crate::kernel::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS_STR};

unsafe fn print_info() {
    x86_printk!("\n");

    if BOOT == "BIOS" {
        kinfo!("VGA text/graphics initialized\n");
        x86_printk!("   Size: {}x{}\n", BUFFER_WIDTH, BUFFER_HEIGHT);
        x86_printk!("   Address: {}\n", VGA_ADDRESS_STR);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    boot_init();
    x86_printk!("Starting kernel...\n");

    print_info();

    x86_kernel_init();

    die()
}
