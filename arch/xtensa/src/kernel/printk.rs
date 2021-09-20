use core::fmt::Arguments;

pub fn _xprintk(fmt: Arguments) {

}

#[macro_export]
macro_rules! xpirntk {
    ($arg:tt) => {crate::kernel::printk::_xprintk(format_args!(*($arg)))};
}
