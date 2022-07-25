/// STM32f407
#[cfg(feature = "stm32f407")]
pub mod board {
    use super::gpio::Stm32F4Gpio;
    use novuskinc::drivers::manager::DRIVER_MANAGER;

    pub unsafe fn stm_board_init() {
        let mut gpio = Stm32F4Gpio::new();
        gpio.enable_gpios();
    }

    pub unsafe fn stm_board_add_drivers() {

    }


}

/// A common GPIO driver
pub mod gpio {
    use stm32f4xx_hal::pac::Peripherals;

    pub struct Stm32F4Gpio {
        pub gpios_enabled: bool,
    }

    impl Stm32F4Gpio {
        pub fn new() -> Self {
            return  Stm32F4Gpio {
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
}
