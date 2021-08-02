use core::fmt::Arguments;
use super::board::write_fmt;

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn _riscv_printk(fmt: Arguments) {
    write_fmt(fmt);
}

#[macro_export]
macro_rules! riscv_printk {
    ($($arg:tt)*) => {$crate::kernel::printk::_riscv_printk(*($arg)*)};
}
