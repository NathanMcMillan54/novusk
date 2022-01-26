use core::fmt::Arguments;

extern "C" {
    pub(crate) fn _early_printk(print: Arguments);
}

#[macro_export]
macro_rules! early_printk {
    ($($arg:tt)*) => { unsafe { $crate::kernel::early_printk::_early_printk($($arg)*); } }
}
