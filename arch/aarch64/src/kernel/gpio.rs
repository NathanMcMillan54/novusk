use arm::rpi::MMIO_BASE;
use tock_registers::registers::ReadWrite;

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

    pub GPPUDCLK0 [
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

pub const GPFSEL1: *const ReadWrite<u32, GPFSEL1::Register> = (MMIO_BASE + 0x0020_0004) as *const ReadWrite<u32, GPFSEL1::Register>;
pub const GPPUD: *const ReadWrite<u32> = (MMIO_BASE + 0x0020_0094) as *const ReadWrite<u32>;
pub const GPPUDCLK0: *const ReadWrite<u32, GPPUDCLK0::Register> = (MMIO_BASE + 0x0020_0098) as *const ReadWrite<u32, GPPUDCLK0::Register>;
