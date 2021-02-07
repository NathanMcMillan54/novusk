use core::fmt::{Arguments, Write};

pub fn _print(args: Arguments) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    arch::x86::x86lib::print::x86_print(format_args!("{}", args));
}

pub fn _printnl(args: Arguments) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    arch::x86::x86lib::print::x86_print(format_args!("{}\n", args));
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        $crate::user::print::_printnl(format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {$crate::user::print::_print(format_args!($($arg)*))};
}
