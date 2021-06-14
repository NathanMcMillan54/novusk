#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {$crate::printk::print::_printk(format_args!($($arg)*))};
}
