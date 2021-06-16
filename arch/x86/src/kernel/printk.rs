use core::fmt::Arguments;
use crate::vga_write;
use crate::boot::boot::boot_method;

#[export_name = "arch_printk"]
#[no_mangle]
pub unsafe extern "C" fn x86_printk(args: Arguments) {
    if boot_method() == "BIOS" {
        vga_write!("{}", args);
    } else if boot_method() == "UEFI" {
        efi_write!("{}", args);
    }
}
