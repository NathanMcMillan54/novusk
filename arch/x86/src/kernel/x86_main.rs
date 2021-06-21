use super::cpu::id::{get_cpuid, BRAND};
use super::kernel::*;
use super::x86_init::x86_init;
use nkuefi::kernel::system_table;
use nkuefi::proto::text::text_mode;
use crate::boot::boot::{boot_init, die, BOOT};
use crate::drivers::vga::{HEIGHT, WIDTH, VGA_ADDRESS_STR};
use crate::include::asm::hlt;

#[no_mangle]
pub unsafe extern "C" fn x86_main() -> ! {
    x86_printk!("");

    kinfo!("Boot initialized");
    x86_printk!("   Boot method: {}", BOOT);

    if BOOT == "BIOS" {
        kinfo!("VGA text/graphics initialized");
        x86_printk!("   Size: {}x{}", WIDTH, HEIGHT);
        x86_printk!("   Address: {}", VGA_ADDRESS_STR);
    } else if BOOT == "UEFI" {
        let mode = text_mode(system_table().as_ref().stdout()).unwrap();

        kinfo!("UEFI stdout initialized");
        x86_printk!("   Text mode: {:?}", mode);
    } else {
        x86_printk!("honestly, how?");
        x86_printk!("how did you boot with {}", BOOT);
        x86_printk!("there's no way!");
        die();
    }

    get_cpuid();
    kinfo!("Got CPU id");
    x86_printk!("   CPU brand: {}", BRAND);

    x86_init();
    hlt();
}
