use core::fmt::Write;
use core::ops::Add;
use asminc::aarch64::io::{inb, outb};
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, DriverResult};
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::Timer;
use crate::bcm2837::SOC_INFO;

pub struct Bcm2837Timer {
    pub interval: u32,
    pub value: u32,
}

impl Bcm2837Timer {
    pub unsafe fn init(&mut self) {
        let peripherals = SOC_INFO.get("Peripheral Address").unwrap();

        let timer = peripherals + SOC_INFO.get("Timer CS Offset").unwrap();
        let timer_cl0 = timer.add(0x4);
        let timer_c1 = timer.add(0x10);

        self.value = inb(timer_cl0);

        self.value += self.interval;

        outb(timer_c1, self.value as u16);
    }
}

#[no_mangle]
pub static mut BCM2837_TIMER: Bcm2837Timer = Bcm2837Timer {
    interval: 0,
    value: 0
};

unsafe fn bcm2837_timer_init() -> u8 {
    BCM2837_TIMER.init();

    if BCM2837_TIMER.value == 0 {
        return 1;
    } else { return 0; }
}

define_kernel_function!(KernelFunctionName::device_timer_init, -> u8, bcm2837_timer_init);
