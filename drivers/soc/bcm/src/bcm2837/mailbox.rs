use core::arch::asm;
use core::ops::Deref;
use novuskinc::firmware::*;
use tock_registers::registers::{ReadOnly, WriteOnly};
use tock_registers::interfaces::{Readable, Writeable};
use crate::bcm2837::SOC_INFO;

register_bitfields! {
    u32,

    STATUS [
        FULL  OFFSET(31) NUMBITS(1) [],
        EMPTY OFFSET(30) NUMBITS(1) []
    ]
}


#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    READ: ReadOnly<u32>,                     // 0x00
    __reserved_0: [u32; 5],                  // 0x04
    STATUS: ReadOnly<u32, STATUS::Register>, // 0x18
    __reserved_1: u32,                       // 0x1C
    WRITE: WriteOnly<u32>,                   // 0x20
}

pub mod channel {
    pub const PROP: u32 = 8;
}

pub mod tag {
    pub const GETSERIAL: u32 = 0x10004;
    pub const SETCLKRATE: u32 = 0x38002;
    pub const SETPHYWH: u32 = 0x48003;
    pub const SETVIRTWH: u32 = 0x48004;
    pub const SETVIRTOFFSET: u32 = 0x48009;
    pub const SETDEPTH: u32 = 0x48005;
    pub const SETPXORDER: u32 = 0x48006;
    pub const GETFB: u32 = 0x40001;
    pub const GETPITCH: u32 = 0x40008;
    pub const LAST: u32 = 0;
}

mod response {
    pub const SUCCESS: u32 = 0x8000_0000;
    pub const ERROR: u32 = 0x8000_0001;
}

pub const REQUEST: u32 = 0;

#[repr(C)]
#[repr(align(16))]
pub struct Bcm2837Mailbox {
    pub buffer: [u32; 36],
}

impl Deref for Bcm2837Mailbox {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Bcm2837Mailbox {
    pub fn new() -> Self {
        Bcm2837Mailbox { buffer: [0; 36] }
    }

    fn ptr() -> *const RegisterBlock {
        let periph_addr = unsafe { SOC_INFO.get("Peripheral Address").unwrap() };
        let video_offset = unsafe { SOC_INFO.get("Video Core Offset").unwrap() };

        let mb_addr = periph_addr + video_offset;

        mb_addr as *const RegisterBlock
    }

    fn wait(&self) {
        loop {
            if self.status() == FMI_STATUS_EMPTY {
                break;
            }
        }
    }
}

impl FirmwareInterface for Bcm2837Mailbox {
    fn init(&mut self) {

    }

    fn name(&self) -> &'static str {
        return "BCM2837 Mailbox";
    }

    fn mb_call(&mut self, channel: u32) -> Result<(), u32> {
        self.wait();

        let buf_ptr = self.buffer.as_ptr() as u32;

        self.WRITE.set((buf_ptr & !0xF) | (channel & 0xF));

        loop {
            loop {
                if self.status() == FMI_STATUS_FULL {
                    break;
                }

                unsafe { asm!("nop") };
            }

            let resp: u32 = self.READ.get();

            if ((resp & 0xF) == channel) && ((resp & !0xF) == buf_ptr) {
                return match self.buffer[1] {
                    response::SUCCESS => Ok(()),
                    response::ERROR => Err(FMI_RESPONSE_ERROR),
                    _ => Err(FMI_RESPONSE_OTHER),
                };
            }
        }
    }

    fn status(&self) -> u32 {
        if self.STATUS.is_set(STATUS::EMPTY) {
            return FMI_STATUS_EMPTY;
        } else { return FMI_STATUS_FULL; }
    }

    fn add_index(&mut self, index: usize, val: u32) {
        self.buffer[index] = val;
    }
}
