use core::ops::Deref;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;
use bcm::bcm2837::SOC_INFO;

register_bitfields! {
    u32,

    /// GPIO Function Select 1
    pub GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD1 = 0b010

        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD1 = 0b010
        ]
    ],

    /// GPIO Function Select 2
    pub GPFSEL2 [
        FSEL29 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    /// GPIO Pull up/down Enable Clock 0
    pub GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],

    /// GPIO Output Set 0
    pub GPSET0 [
        O29 OFFSET(29) NUMBITS(1) [
            Clear = 0,
            Set = 1
        ]
    ],

    /// GPIO Output Clear 0
    pub GPCLR0 [
        O29 OFFSET(29) NUMBITS(1) [
            Set = 0,
            Clear = 1
        ]
    ],
}

pub const GPFSEL1: *const ReadWrite<u32, GPFSEL1::Register> = (0x3F00_0000 + 0x0020_0004) as *const ReadWrite<u32, GPFSEL1::Register>;
pub const GPPUD: *const ReadWrite<u32> = (0x3F00_0000 + 0x0020_0094) as *const ReadWrite<u32>;
pub const GPPUDCLK0: *const ReadWrite<u32, GPPUDCLK0::Register> = (0x3F00_0000 + 0x0020_0098) as *const ReadWrite<u32, GPPUDCLK0::Register>;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub __GPFSEL0: u32,                                 // 0x00
    pub __GPFSEL1: u32,                                 // 0x04
    pub GPFSEL2: ReadWrite<u32, GPFSEL2::Register>, // 0x08
    pub __GPFSEL3: u32,                                 // 0x0C
    pub __GPFSEL4: u32,                                 // 0x10
    pub __GPFSEL5: u32,                                 // 0x14
    __reserved_0: u32,                              // 0x18
    pub GPSET0: ReadWrite<u32, GPSET0::Register>,   // 0x1C
    __GPSET1: u32,                                  // 0x20
    __reserved_1: u32,                              // 0x24
    pub GPCLR0: ReadWrite<u32, GPCLR0::Register>,   // 0x28
}

pub struct Rpi3Gpio;

impl Deref for Rpi3Gpio {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Rpi3Gpio {
    pub fn new() -> Self {
        return Rpi3Gpio;
    }

    pub fn ptr() -> *const RegisterBlock {
        let peripheral = unsafe { SOC_INFO.get("Peripheral Address").unwrap() };
        let gpio_offset = unsafe { SOC_INFO.get("GPIO Offset").unwrap() };
        let gpio_base = peripheral + gpio_offset;

        return gpio_base as *const _;
    }
}

pub mod led {
    use core::fmt::Write;
    use super::*;
    use tock_registers::interfaces::{Readable, Writeable};
    use novuskinc::console::KernelConsoleDriver;
    use novuskinc::drivers::{Driver, DriverResult};
    use novuskinc::fb::FrameBufferGraphics;
    use novuskinc::keyboard::KeyboardInput;
    use novuskinc::led::Led;

    pub struct Rpi3Led {
        pub gpio: Rpi3Gpio,
    }

    impl Rpi3Led {
        pub fn new() -> Self {
            return Rpi3Led {
                gpio: Rpi3Gpio::new(),
            };
        }

        pub fn led_init(&self) {
            self.gpio.GPFSEL2.write(GPFSEL2::FSEL29::Output);
        }

        pub fn led_on(&self) {
            self.gpio.GPSET0.write(GPSET0::O29::Set);
        }

        pub fn led_off(&self) {
            self.gpio.GPCLR0.write(GPCLR0::O29::Clear);
        }
    }

    impl Write for Rpi3Led {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            todo!()
        }
    }

    impl KernelConsoleDriver for Rpi3Led {}

    impl FrameBufferGraphics for Rpi3Led {}

    impl KeyboardInput for Rpi3Led {}

    impl Led for Rpi3Led {
        fn on(&mut self) {
            self.led_on();
        }

        fn off(&mut self) {
            self.led_off();
        }

        fn status(&self) -> u32 {
            0
        }
    }

    impl Driver for Rpi3Led {
        fn driver_name(&self) -> &'static str {
            return "RPi3 Led Driver";
        }

        fn name(&self) -> &'static str {
            return "Led Driver";
        }

        fn init(&self) -> Option<DriverResult> {
            self.led_init();

            if self.gpio.GPFSEL2.matches_any(GPFSEL2::FSEL29::Output) {
                return Some(Ok(()));
            } else { return Some(Err("Failed to set GPFSEL2 value")); }
        }
    }
}
