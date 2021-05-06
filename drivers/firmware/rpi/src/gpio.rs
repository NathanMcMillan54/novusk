use core::ops;
use register::mmio::*;
use crate::board::MMIO_BASE;

register_bitfields! {
    u32,

    GPFSEL2 [
        FSEL29 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    GPSET0 [
        O29 OFFSET(29) NUMBITS(1) [
            Clear = 0,
            Set = 1
        ]
    ],

    GPCLR0 [
        O29 OFFSET(29) NUMBITS(1) [
            Set = 0,
            Clear = 1
        ]
    ]
}

const GPIO_BASE: u32 = MMIO_BASE + 0x20_0000;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    __GPFSEL0: u32,                                 // 0x00
    __GPFSEL1: u32,                                 // 0x04
    pub GPFSEL2: ReadWrite<u32, GPFSEL2::Register>, // 0x08
    __GPFSEL3: u32,                                 // 0x0C
    __GPFSEL4: u32,                                 // 0x10
    __GPFSEL5: u32,                                 // 0x14
    __reserved_0: u32,                              // 0x18
    pub GPSET0: ReadWrite<u32, GPSET0::Register>,   // 0x1C
    __GPSET1: u32,                                  // 0x20
    __reserved_1: u32,                              // 0x24
    pub GPCLR0: ReadWrite<u32, GPCLR0::Register>,   // 0x28
}

pub struct Gpio;

impl ops::Deref for Gpio {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Gpio {
    fn ptr() -> *const RegisterBlock {
        GPIO_BASE as *const _
    }
}
