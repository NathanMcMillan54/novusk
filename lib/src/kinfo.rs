use core::fmt::{Arguments, Write};
use crate::kprint::_kprint;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use arch::x86::x86lib::print::*;

extern "C" {
    fn kernel_time() -> f32;
}

pub fn _kinfo(arg: Arguments) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    arch::x86::x86lib::print::x86_print(format_args!("[ {} ] {}", unsafe { kernel_time() }, arg));
}

#[macro_export]
macro_rules! kinfo {
    ($($arg:tt)*) => {$crate::kinfo::_kinfo(format_args!($($arg)*))};
}
