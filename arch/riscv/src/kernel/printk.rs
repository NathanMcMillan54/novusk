use core::fmt::Arguments;

#[no_mangle]
#[export_name = "arch_printk"]
pub fn _rv_printk(fmt: Arguments) -> Arguments {
    #[cfg(any(feature = "hifive", feature = "lofive"))]
    sifive::sprint!("{}", fmt);

    return fmt;
}

#[no_mangle]
pub extern "C" fn kmain_printk(fmt: Arguments) {
    _rv_printk(fmt);
}

#[macro_export]
macro_rules! rv_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_rv_printk(format_args!($($arg)*))};
}
