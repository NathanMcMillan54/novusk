#![no_std]

extern crate printk;
pub use printk::_printk;

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::_printk(format_args!($($arg)*))};
}
