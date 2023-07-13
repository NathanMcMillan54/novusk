use cortex_m::peripheral::{Peripherals, syst::SystClkSource};
use cortex_m_rt::{exception, ExceptionFrame};

pub fn sys_time_setup() {
    let mut cpu_peripherals = Peripherals::take().unwrap();

    let mut syst = cpu_peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);

    // Reloads ~100ms
    syst.set_reload(800000000);
    syst.enable_counter();
    syst.enable_interrupt();
}

#[exception]
fn SysTick() {

}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    // TODO: When ``IrqChip`` and ``IrqHandler`` from ``novuskinc`` are implemented call
    // ``handle_irq(irqn)``
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    super::print::_early_print(format_args!("{}{:?}", "\n----- HardFault: -----\n", ef));
    loop { }
}
