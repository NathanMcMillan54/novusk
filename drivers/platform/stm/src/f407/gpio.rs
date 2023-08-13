use stm32f4xx_hal::pac::Peripherals;

lazy_static::lazy_static! {
    static ref F4GPIO: spin::Mutex<F4Gpio> = spin::Mutex::new(F4Gpio::new());
}

pub struct F4Gpio {
    pub gpios_enabled: bool,
}

impl F4Gpio {
    pub fn new() -> Self {
        return F4Gpio {
            gpios_enabled: Self::check_enabled(),
        };
    }

    /// Enables GPIOA, GPIOB, GPIOC, and GPIOD.
    pub fn enable_gpios(&mut self) {
        if self.gpios_enabled {
            return;
        }

        let peripherals = unsafe { Peripherals::steal() };
        let rcc = peripherals.RCC;

        rcc.ahb1enr.modify(|r, w| {
            w.gpioaen().enabled();
            w.gpioben().enabled();
            w.gpiocen().enabled();
            w.gpioden().enabled()
        });

        self.gpios_enabled = Self::check_enabled();
    }

    /// Checks if GPIOA, GPIOB, GPIOC, and GPIOD are enabled.
    pub fn check_enabled() -> bool {
        let peripherals = unsafe { Peripherals::steal() };
        let rcc = peripherals.RCC;

        if rcc.ahb1enr.read().gpioaen().is_disabled() {
            return false;
        } else if rcc.ahb1enr.read().gpioben().is_disabled() {
            return false;
        } else if rcc.ahb1enr.read().gpiocen().is_disabled() {
            return false;
        } else if rcc.ahb1enr.read().gpioden().is_disabled() {
            return false;
        } else { return true; }
    }
}

pub(crate) fn gpio_init() {
    F4GPIO.lock().enable_gpios();
}
