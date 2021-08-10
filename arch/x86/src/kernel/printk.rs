use core::fmt::Arguments;
use super::vga::_vga_print;
use nkuefi::x64_uefi::_serial_print;
use crate::boot::boot::BOOT;

#[export_name = "arch_printk"]
#[no_mangle]
pub unsafe extern "C" fn _x86_printk(fmt: Arguments) {
    if BOOT == "BIOS" {
        _vga_print(format_args!("{}{}", fmt, "\n"));
    } else if BOOT == "UEFI" {
        _serial_print(fmt);
    } else {
        // \_('_')_/ <( idk? )
    }
}

#[macro_export]
macro_rules! x86_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_x86_printk(format_args!($($arg)*))};
}
