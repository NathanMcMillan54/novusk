use core::ops::Deref;
use super::bases::*;
use libbmu::Time;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::*;

register_bitfields! {
    u32,

    pub STATUS [
        FULL OFFSET(31) NUMBITS(1) [],
        EMPTY OFFSET(30) NUMBITS(1) []
    ]
}

#[derive(Copy, Clone, PartialEq)]
#[repr(usize)]
pub enum RpiMb {
    MboxRequest = 0,
    MboxSuccess = 0x8000_0000,
    MboxError = 0x8000_0001,
}

#[derive(Copy, Clone, PartialEq)]
pub enum RpiMbCh {
    MboxChPower = 0,
    MboxChFb = 1,
    MboxChVuart = 2,
    MboxChVChiq = 3,
    MnoxChLeds  = 4,
    MboxChBtns  = 5,
    MboxChTouch = 6,
    MboxChCount = 7,
    MboxChProp  = 8
}

#[derive(Copy, Clone, PartialEq)]
pub enum RpiMboxTag {
    MboxTagSetPower = 0x28001,
    MboxTagSetClkrate = 0x38002,
    MboxTagSetPhywh   = 0x48003,
    MboxTagSetVirtwh  = 0x48004,
    MboxTagSetVirtoff = 0x48009,
    MboxTagSetSetdepth  = 0x48005,
    MboxTagSetPxlordr = 0x48006,
    MboxTagGetFb = 0x40001,
    MboxTagGetPitch = 0x40008,
    MboxTagGetSerial = 0x10004,
    MboxTagLast = 0
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

const SUCCESS: usize = RpiMb::MboxSuccess as usize;
const ERROR: usize = RpiMb::MboxError as usize;

pub struct MailBox {
    pub mb_buffer: [usize; 36],
}

impl Deref for MailBox {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        return unsafe { &*MailBox::ptr() };
    }
}

impl MailBox {
    pub fn new() -> Self {
        return MailBox { mb_buffer: [0; 36] }
    }

    pub fn clear(&mut self) {
        for i in 0..36 {
            self.mb_buffer[i] = 0;
        }
    }

    fn ptr() -> *const RegisterBlock {
        return VIDEOCORE_MBOX as *const RegisterBlock;
    }

    pub fn call(&mut self, channel: u32) -> i32 {
        let mut time = Time::new();

        loop {
            if !self.STATUS.is_set(STATUS::FULL) {
                break;
            }
        }

        let buffer = self.mb_buffer.as_ptr() as u32;

        self.WRITE.set((buffer & !0xF) | (channel & 0xF));

        loop {
            loop {
                if !self.STATUS.is_set(STATUS::EMPTY) {
                    break;
                }

                time.sleepc(1);
            }

            let resp = self.READ.get();

            if ((resp & 0xF) == channel) && ((resp & !0xF) == buffer) {
                match self.mb_buffer[1] {
                    SUCCESS =>
                        return 5,
                    ERROR =>
                        return 0,
                    _ =>
                        return 1,
                }
            }
        }
    }
}
