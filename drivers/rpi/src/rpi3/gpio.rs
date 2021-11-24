use core::ops::Deref;
use crate::{MMIO_BASE, GPIO_BASE};
use crate::{Rpi3, RaspberryPi};
use tock_registers::registers::ReadWrite;
use tock_registers::interfaces::Writeable;

register_bitfields! {
    u32,

    pub GPFSEL1 [
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD1 = 0b010

        ],

        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD1 = 0b010
        ]
    ],

    pub GPFSEL2 [
        FSEL29 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    pub GPPUDCLK0 [
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],

    pub GPSET0 [
        O29 OFFSET(29) NUMBITS(1) [
            Clear = 0,
            Set = 1
        ]
    ],

    pub GPCLR0 [
        O29 OFFSET(29) NUMBITS(1) [
            Set = 0,
            Clear = 1
        ]
    ],
}

pub const GPFSEL1: *const ReadWrite<u32, GPFSEL1::Register> = (MMIO_BASE + 0x0020_0004) as *const ReadWrite<u32, GPFSEL1::Register>;
pub const GPPUD: *const ReadWrite<u32> = (MMIO_BASE + 0x0020_0094) as *const ReadWrite<u32>;
pub const GPPUDCLK0: *const ReadWrite<u32, GPPUDCLK0::Register> = (MMIO_BASE + 0x0020_0098) as *const ReadWrite<u32, GPPUDCLK0::Register>;

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
        return GPIO_BASE as *const _;
    }
}
