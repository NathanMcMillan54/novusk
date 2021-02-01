use arch::ARCH;
use core::fmt::{Arguments, Write};
use crate::kprint::_kprint;

fn x86_kinfo(args: Arguments) {
    use arch::x86::lib::print::*;
    use arch::x86::kernel::time::time::kernel_time;
    write!(WRITER.lock(), "[ {} ] {}", kernel_time(), args).unwrap();
}

pub fn _kinfo(arg: Arguments) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    x86_kinfo(arg)
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::kinfo::_kinfo(format_args!($($arg)*))};
}
