// For printing in probe-run on micro controllers and simple boards
use core::fmt::Arguments;

pub fn _dprint(fmt: Arguments) {
    defmt::info!("{}{}", "DEBUG", fmt);
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {$crate::kernel::debug::_dprint(format_args!($($arg)*))};
}
