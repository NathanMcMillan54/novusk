// For printing in probe-run on micro controllers and simple boards
use core::fmt::Arguments;

pub fn dprint(fmt: Arguments) {
    defmt::info!("{}", fmt);
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {$crate::kernel::debug::_arm32_dprint(format_args!($($arg)*))};
}
