use bootloader::BootInfo;
use super::boot::{die, boot_init, BOOT};
use core::ptr::write_volatile;
use kinfo::status::KStatus;
use crate::kernel::kernel::*;
use crate::kernel::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS_STR};
use crate::kinfo::InfoDisplay;

unsafe fn print_info() {
    early_printk!("\n");

    if BOOT == "BIOS" {
        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: "VGA text/graphics initialized",
            messages: None,
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    boot_init();
    early_printk!("Starting kernel...\n");

    print_info();

    x86_kernel_init();

    die()
}
