use core::fmt::{Arguments, Write};

extern "C" { fn red(arg: Arguments); }

pub unsafe fn _panic(info: Arguments) -> Arguments {
    red(info);
    return info;
}

#[macro_export]
macro_rules! panick {
    ($($arg:tt)*) => {$crate::panic::_panic(format_args!($($arg)*))};
}

