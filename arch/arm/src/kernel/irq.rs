pub unsafe fn setup_irqs() {
    #[cfg(feature = "cortex_m")]
    cm_ints::cortex_m_irq_setup();

    #[cfg(feature = "cortex_a")]
    ca_ints::ca_irqs_setup();
}

pub unsafe fn irqs_init() {
    #[cfg(feature = "cortex_m")]
    cm_ints::cortex_m_irq_init();

    extern "C" {
        fn device_specific_irqs_init();
    }

    device_specific_irqs_init();
}

#[cfg(feature = "cortex_m")]
mod cm_ints {
    use core::arch::asm;
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

#[cfg(feature = "cortex_a")]
mod ca_ints {
    #[cfg(feature = "cortex_a")]
    fn cortex_a32_irq_setup() {

    }

    pub fn ca_irqs_setup() {
        #[cfg(target_arch = "arm")]
        cortex_a32_irq_setup();

        #[cfg(target_arch = "aarch64")]
        unsafe { crate::bits64::arm64_irq_setup(); }
    }
}
