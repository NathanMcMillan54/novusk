use bootloader::BootInfo;
use super::boot::{die, boot_init, BOOT};
use core::ptr::write_volatile;
use crate::include::asm::hlt;
use crate::kernel::kernel::*;
use crate::kernel::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS_STR};
use crate::mm::early_memory_init;

unsafe fn print_info() {
    x86_printk!("");

    if BOOT == "BIOS" {
        kinfo!("VGA text/graphics initialized");
        x86_printk!("   Size: {}x{}", BUFFER_WIDTH, BUFFER_HEIGHT);
        x86_printk!("   Address: {}", VGA_ADDRESS_STR);
    }

    x86_printk!("Boot took: {} seconds", amd64_timer::ticks() as f64 / 1000000000.0);
}

#[cfg(not(feature = "grub"))]
#[no_mangle]
pub unsafe extern "C" fn _start(bootinfo: &'static BootInfo) -> ! {
    boot_init();
    x86_printk!("Starting kernel...");

    print_info();

    early_memory_init(bootinfo);
    kinfo!("Early memory initialized");

    x86_kernel_init();

    die()
}

#[no_mangle]
pub extern "C" fn grub_start_novusk() -> ! {
    loop {  }
}
