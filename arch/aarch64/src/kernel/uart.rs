use core::fmt::{Arguments, Result, Write};
use core::ops;
use super::gpio;
use rpi::MMIO_BASE;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

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

const MINI_UART_BASE: u32 = MMIO_BASE + 0x21_5000;

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

pub struct Uart;

impl ops::Deref for Uart {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Uart {
    // Because the Aarch64 kernel is for the RPi you won't set the uart address in the "new"
    // function like in RISCV, if a new board is supported it'll take arguments.
    pub fn new() -> Self {
        return Uart;
    }

    fn ptr() -> *const RegisterBlock {
        MINI_UART_BASE as *const _
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

        unsafe {
            (*gpio::GPFSEL1).modify(gpio::GPFSEL1::FSEL14::TXD1 + gpio::GPFSEL1::FSEL15::RXD1);

            (*gpio::GPPUD).set(0);
            for _ in 0..150 {
                llvm_asm!("nop" :::: "volatile");
            }

            (*gpio::GPPUDCLK0).write(
                gpio::GPPUDCLK0::PUDCLK14::AssertClock + gpio::GPPUDCLK0::PUDCLK15::AssertClock,
            );
            for _ in 0..150 {
                llvm_asm!("nop" :::: "volatile");
            }

            (*gpio::GPPUDCLK0).set(0);
        }

        self.AUX_MU_CNTL.write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);
    }

    pub fn send(&self, c: char) {
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::TX_EMPTY) {
                break;
            }

            unsafe { llvm_asm!("nop" :::: "volatile") };
        }

        self.AUX_MU_IO.set(c as u32);
    }

    pub fn receive(&mut self) -> u8 {
        let mut ret = self.AUX_MU_IO.get() as u8;

        return ret;
    }

    pub fn input_char(&self) -> char {
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::DATA_READY) {
                break;
            }

            unsafe { llvm_asm!("nop" :::: "volatile") };
        }

        let mut ret = self.AUX_MU_IO.get() as u8 as char;

        if ret == '\r' {
            ret = '\n'
        }

        return ret;
    }

    pub fn write_string(&self, string: &str) {
        for c in string.chars() {
            if c == '\n' {
                self.send('\r')
            }

            self.send(c);
        }
    }
}

impl Write for Uart {
    fn write_str(self: &mut Self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}
