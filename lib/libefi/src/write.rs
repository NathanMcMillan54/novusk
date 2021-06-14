use core::fmt::{Arguments, Write};
use nkuefi::st::st;

pub unsafe fn _efi_write(fmt: Arguments) {
    write!(st().as_ref().stdout(), "{}{}", fmt, "\n");
}

#[macro_export]
macro_rules! efi_write {
    ($($arg:tt)*) => {$crate::write::_efi_write(format_args!($($arg)*))};
}

