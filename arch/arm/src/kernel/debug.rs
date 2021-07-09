// For printing in qemu or probe-run on micro controllers and simple boards

#[cfg(target_arch = "arm")]
pub use crate::kernel::printk::arm_printk as _arm32_dprint;
use core::fmt::Arguments;

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {$crate::kernel::debug::_arm32_dprint(format_args!($($arg)*))};
}
