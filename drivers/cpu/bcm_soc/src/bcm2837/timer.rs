use core::fmt::Write;
use core::ops::Add;
use asminc::aarch64::io::{inb, outb};
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::drivers::manager::DEVICE_DRIVERS;
use novuskinc::drivers::names::TIMER;
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::{Serial, Storage, Timer};
use crate::bcm2837::SOC_INFO;

pub struct Bcm2837Timer {
    interval: u32,
    value: u32,
}

impl Bcm2837Timer {
    pub unsafe fn _init(&mut self) {
        let peripherals = SOC_INFO.get("Peripheral Address");

        let timer = peripherals + SOC_INFO.get("Timer CS Offset");
        let timer_cl0 = timer.add(0x4);
        let timer_c1 = timer.add(0x10);

        self.value = inb(timer_cl0);

        self.value += self.interval;

        outb(timer_c1, self.value as u16);
    }
}

impl Timer for Bcm2837Timer {
    fn value(&self) -> u32 {
        self.value
    }

    fn interval(&self) -> u32 {
        self.interval
    }
}

impl KernelConsoleDriver for Bcm2837Timer {}

impl FrameBufferGraphics for Bcm2837Timer {}

impl KeyboardInput for Bcm2837Timer {}

impl Storage for Bcm2837Timer {}

impl Serial for Bcm2837Timer {}

impl Write for Bcm2837Timer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Led for Bcm2837Timer {}

impl Driver for Bcm2837Timer {
    fn driver_name(&self) -> &'static str {
        "BCM2837 Timer"
    }

    fn name(&self) -> &'static str {
        TIMER
    }

    fn init(&mut self) -> DriverResult {
        Ok(())
    }
}

#[no_mangle]
pub static mut BCM2837_TIMER: Bcm2837Timer = Bcm2837Timer {
    interval: 0,
    value: 0
};

unsafe fn bcm2837_timer_init() -> u8 {
    DEVICE_DRIVERS.add_driver(&mut BCM2837_TIMER as &mut dyn Driver);

    BCM2837_TIMER.init();

    if BCM2837_TIMER.value() == 0 {
        return 1;
    } else { return 0; }
}

define_kernel_function!(KernelFunctionName::device_timer_init, -> u8, bcm2837_timer_init);
