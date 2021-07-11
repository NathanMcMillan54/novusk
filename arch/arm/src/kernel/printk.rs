use core::fmt::Arguments;
use cortex_m_semihosting::hprint;
use super::kernel::dprint;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn arm_printk(fmt: Arguments) {
    dprint!("{}{}", fmt, "\n");
    hprint!("{}{}", fmt, "\n");
}

#[macro_export]
macro_rules! arm32_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::arm_printk(format_args!($($arg)*))};
}
