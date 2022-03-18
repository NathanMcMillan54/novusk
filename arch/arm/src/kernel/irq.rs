pub unsafe fn setup_irqs() {
    #[cfg(feature = "cortex_m")]
    cm_ints::cortex_m_irq_setup();
}

pub unsafe fn irqs_init() {
    #[cfg(feature = "cortex_m")]
    cm_ints::cortex_m_irq_init();
}

#[cfg(feature = "cortex_m")]
mod cm_ints {
    use cortex_m::peripheral::syst::SystClkSource;
    use cortex_m::Peripherals;
    use cortex_m_rt::{exception, ExceptionFrame};

    pub unsafe fn cortex_m_irq_setup() {
        let cpu_peripherals = Peripherals::steal();
        let mut syst = cpu_peripherals.SYST;

        // Setup system timer for IRQs
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(8_000_000);
    }

    pub unsafe fn cortex_m_irq_init() {
        let cpu_peripherals = Peripherals::steal();
        let mut syst = cpu_peripherals.SYST;

        syst.enable_counter();
        syst.enable_interrupt();
    }

    #[exception]
    unsafe fn DefaultHandler(irq: i16) -> ! {
        hprintln!("IRQ: {}", irq);
        loop { asm!("wfe"); }
    }

    #[exception]
    unsafe fn SysTick() {

    }
}
