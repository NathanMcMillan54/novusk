use bootloader::bootinfo::FrameRange;
use super::boot::{die, boot_init, BOOT};
use crate::drivers::vga::{HEIGHT, WIDTH, VGA_ADDRESS_STR};
use crate::kernel::kernel::*;

unsafe fn print_info() {
    x86_printk!("");

    if BOOT == "BIOS" {
        kinfo!("VGA text/graphics initialized");
        x86_printk!("   Size: {}x{}", WIDTH, HEIGHT);
        x86_printk!("   Address: {}", VGA_ADDRESS_STR);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    boot_init();
    x86_printk!("Starting kernel...");

    print_info();
    x86_kernel_init();

    die()
}
