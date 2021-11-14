use cortex_m::Peripherals;
use cortex_m::peripheral::syst::SystClkSource;

pub mod common;
pub use common::*;

pub(crate) fn int_init() {
    let peripherals = unsafe { Peripherals::steal() };
    let mut syst = peripherals.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_counter();
    syst.enable_interrupt();
}
