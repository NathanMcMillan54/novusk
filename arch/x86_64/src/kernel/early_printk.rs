use core::fmt::{Arguments, Write};
use super::video_vga::EARLY_VGA;
use x86_64::instructions::interrupts::without_interrupts;

#[no_mangle]
pub unsafe extern "C" fn _early_printk(fmt: Arguments) {
    without_interrupts(||{
        EARLY_VGA.lock().write_fmt(fmt);
    });
}

#[macro_export]
macro_rules! early_printk {
    ($($arg:tt)*) => { unsafe { $crate::kernel::early_printk::_early_printk(format_args!($($arg)*)); } }
}
