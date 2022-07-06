use cortex_m_rt::exception;

#[cfg(feature = "debug")]
use core::fmt::Write;

pub(crate) static mut COUNTED_SYSTICKS: u64 = 0;

#[exception]
unsafe fn SysTick() {
    COUNTED_SYSTICKS += 1;

    #[cfg(feature = "debug")]
    hio::io::HioWriter::new().write_fmt(format_args!("{}{}{}", "SysTick invoked, COUNTED_SYSTICKS = ", COUNTED_SYSTICKS, "\n"));
}
