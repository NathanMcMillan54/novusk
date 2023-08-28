use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{add_driver, Driver, DriverResult};
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::{Serial, Storage, Timer};
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::pac::Peripherals;

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
            //w.gpioden().set_bit()
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

impl KernelConsoleDriver for F4Gpio {}

impl FrameBufferGraphics for F4Gpio {}

impl KeyboardInput for F4Gpio {}

impl Storage for F4Gpio {}

impl Serial for F4Gpio {}

impl Write for F4Gpio {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Led for F4Gpio {
    fn on(&mut self) {
        let mut peripherals = unsafe { Peripherals::steal() };
        let mut gpiod = peripherals.GPIOD.split();
        let mut d13 = gpiod.pd13.into_push_pull_output();
        d13.set_high();
    }

    fn off(&mut self) {
        let mut peripherals = unsafe { Peripherals::steal() };
        let mut gpiod = peripherals.GPIOD.split();
        let mut d13 = gpiod.pd13.into_push_pull_output();
        d13.set_low();
    }
}

impl Timer for F4Gpio {}

impl Driver for F4Gpio {
    fn driver_name(&self) -> &'static str {
        "STM32F4 GPIO"
    }

    fn init(&mut self) -> DriverResult {
        unsafe { self.enable_gpios(); }

        return if self.gpios_enabled {
            Ok(())
        } else { Err("Failed to enable GPIO driver") }
    }
}

pub(crate) unsafe fn gpio_init() {
    static mut F4GPIO: F4Gpio = F4Gpio {
        gpios_enabled: false,
    };

    add_driver(&mut F4GPIO as &mut dyn Driver);
}
