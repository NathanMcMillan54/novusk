use core::fmt::{Arguments, Write};

extern "C" { fn add_0_1(); }

pub fn _kprint(arg: Arguments) {
    unsafe { add_0_1(); }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    arch::x86::x86lib::print::x86_print(arg);
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {$crate::kprint::_kprint(format_args!($($arg)*))};
}
