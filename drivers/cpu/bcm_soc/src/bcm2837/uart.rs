use core::arch::asm;
use core::fmt::{Arguments, Write};
use core::ops;
use novuskinc::drivers::{names::SERIAL, Driver, DriverResult};
use novuskinc::prelude::*;
use super::gpio;
use tock_registers::interfaces::{Readable, ReadWriteable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use crate::SOC_INFO;

pub unsafe fn bcm2837_uart_init() {
    static UART: Bcm2837Uart = Bcm2837Uart;
    UART.init();

    //DEVICE_DRIVERS.add_driver(&UART as &'static dyn Driver);
}

register_bitfields! {
    u32,

    AUX_ENABLES [
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    AUX_MU_IIR [
        FIFO_CLEAR OFFSET(1) NUMBITS(2) [
            Rx = 0b01,
            Tx = 0b10,
            All = 0b11
        ]
    ],

    AUX_MU_LCR [
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],

    AUX_MU_LSR [
        TX_EMPTY OFFSET(5) NUMBITS(1) [],
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],

    AUX_MU_CNTL [
        TX_EN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        RX_EN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    AUX_MU_BAUD [
        RATE OFFSET(0) NUMBITS(16) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    __reserved_0: u32,                                  // 0x00
    AUX_ENABLES: ReadWrite<u32, AUX_ENABLES::Register>, // 0x04
    __reserved_1: [u32; 14],                            // 0x08
    AUX_MU_IO: ReadWrite<u32>,                          // 0x40 Uart I/O data
    AUX_MU_IER: WriteOnly<u32>,                         // 0x44
    AUX_MU_IIR: WriteOnly<u32, AUX_MU_IIR::Register>,   // 0x48
    AUX_MU_LCR: WriteOnly<u32, AUX_MU_LCR::Register>,   // 0x4C
    AUX_MU_MCR: WriteOnly<u32>,                         // 0x50
    AUX_MU_LSR: ReadOnly<u32, AUX_MU_LSR::Register>,    // 0x54
    __reserved_2: [u32; 2],                             // 0x58
    AUX_MU_CNTL: WriteOnly<u32, AUX_MU_CNTL::Register>, // 0x60
    __reserved_3: u32,                                  // 0x64
    AUX_MU_BAUD: WriteOnly<u32, AUX_MU_BAUD::Register>, // 0x68
}

pub struct Bcm2837Uart;

impl ops::Deref for Bcm2837Uart {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Bcm2837Uart {
    pub fn new() -> Self {
        return Bcm2837Uart;
    }

    fn ptr() -> *const RegisterBlock {
        let uart_addr = SOC_INFO.get("Peripheral") + SOC_INFO.get("Mini Uart");
        uart_addr as *const _
    }

    pub fn init(&self) {
        self.AUX_ENABLES.modify(AUX_ENABLES::MINI_UART_ENABLE::SET);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_CNTL.set(0);
        self.AUX_MU_LCR.write(AUX_MU_LCR::DATA_SIZE::EightBit);
        self.AUX_MU_MCR.set(0);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_IIR.write(AUX_MU_IIR::FIFO_CLEAR::All);
        self.AUX_MU_BAUD.write(AUX_MU_BAUD::RATE.val(270));

        // Map UART1 to GPIO pins
        unsafe {
            (*gpio::_GPFSEL1()).modify(gpio::GPFSEL1::FSEL14::TXD1 + gpio::GPFSEL1::FSEL15::RXD1);

            (*gpio::_GPPUD()).set(0); // enable pins 14 and 15
            for _ in 0..150 {
                asm!("nop");
            }

            (*gpio::_GPPUDCLK0()).write(
                gpio::GPPUDCLK0::PUDCLK14::AssertClock + gpio::GPPUDCLK0::PUDCLK15::AssertClock,
            );
            for _ in 0..150 {
                asm!("nop");
            }

            (*gpio::_GPPUDCLK0()).set(0);
        }

        self.AUX_MU_CNTL
            .write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);
    }
}

impl Write for Bcm2837Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write(*b);
        }

        Ok(())
    }
}

impl KernelConsoleDriver for Bcm2837Uart {}

impl FrameBufferGraphics for Bcm2837Uart {}

impl KeyboardInput for Bcm2837Uart {}

impl Storage for Bcm2837Uart {}

impl Serial for Bcm2837Uart {
    fn read(&self) -> u8 {
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::DATA_READY) {
                break;
            }

            unsafe { asm!("nop") };
        }

        let mut ret = self.AUX_MU_IO.get() as u8 as char;

        if ret == '\r' {
            ret = '\n'
        }

        return ret as u8;
    }

    fn write(&self, byte: u8) {
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::TX_EMPTY) {
                break;
            }

            unsafe { asm!("nop") };
        }

        self.AUX_MU_IO.set(byte as u32);
    }
}

impl Led for Bcm2837Uart {}

impl Driver for Bcm2837Uart {
    fn driver_name(&self) -> &'static str {
        return "RPi3 UART";
    }

    fn name(&self) -> &'static str {
        return SERIAL;
    }

    fn init(&self) -> DriverResult {

        Ok(())
    }
}
