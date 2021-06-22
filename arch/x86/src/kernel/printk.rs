use core::fmt::Arguments;
use nkuefi::x86::write::_efi_write;
use crate::boot::boot::BOOT;
use crate::drivers::vga::_vga_print;

#[export_name = "arch_printk"]
#[no_mangle]
pub unsafe extern "C" fn _x86_printk(fmt: Arguments) {
    if BOOT == "BIOS" {
        _vga_print(format_args!("{}{}", fmt, "\n"));
    } else if BOOT == "UEFI" {
        _efi_write(fmt);
    } else {
        // \_('_')_/ idk
    }
}

#[macro_export]
macro_rules! x86_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_x86_printk(format_args!($($arg)*))};
}
