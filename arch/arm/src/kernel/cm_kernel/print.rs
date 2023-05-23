use core::fmt::Arguments;

#[cfg(feature = "debug")]
pub(crate) fn _debug_print(args: Arguments) {
    cortex_m_semihosting::hprint!("DEBUG_PIRNT: {}", args);
}

pub(crate) fn _early_print(args: Arguments) {
    #[cfg(feature = "debug")]
    _debug_print(args);
}
