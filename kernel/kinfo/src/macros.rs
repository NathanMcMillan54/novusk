use core::fmt::Arguments;
use crate::status::{set_status, KSTATUS};

pub unsafe fn _kinfo(msg: Arguments) {
    printk!("INFO [ {} ] {}", KSTATUS, msg);
    set_status("ol");
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::macros::_kinfo(format_args!($($arg)*))};
}
