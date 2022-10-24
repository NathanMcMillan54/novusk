use core::arch::asm;
use core::cell::Cell;
use core::fmt::Write;
use core::ops;
use core::sync::atomic::{compiler_fence, Ordering};
use novuskinc::drivers::Driver;
use novuskinc::prelude::*;
use tock_registers::registers::{ReadOnly, WriteOnly};
use tock_registers::interfaces::{Readable, Writeable};
use super::SOC_INFO;

pub mod channel {
    #[no_mangle]
    pub static PROP: u32 = 8;
}

pub mod tag {
    #[no_mangle]
    pub static GET_SERIAL: u32 = 0x10004;
    #[no_mangle]
    pub static SET_CLKRATE: u32 = 0x38002;
    #[no_mangle]
    pub static SET_PHYSWH: u32 = 0x48003;
    #[no_mangle]
    pub static SET_VIRTWH: u32 = 0x48004;
    #[no_mangle]
    pub static SET_VIRTOFFSET: u32 = 0x48009;
    #[no_mangle]
    pub static SET_DEPTH: u32 = 0x48005;
    #[no_mangle]
    pub static SET_PXLORDER: u32 = 0x48006;
    #[no_mangle]
    pub static GET_FB: u32 = 0x40001;
    #[no_mangle]
    pub static GET_PITCH: u32 = 0x40008;
    #[no_mangle]
    pub static LAST: u32 = 0;
}

mod response {
    #[no_mangle]
    pub static SUCCESS: u32 = 0x8000_0000;
    #[no_mangle]
    pub static ERROR: u32 = 0x8000_0001;
}

#[no_mangle]
pub static REQUEST: u32 = 0;
/*
static mut BCM2837_MB: Bcm2837Mb = Bcm2837Mb {
    mb_buffer: Cell::new([0; 36]),
};

#[no_mangle]
unsafe extern "C" fn set_mb_index(index: usize, value: u32) {
    let mut buffer = BCM2837_MB.mb_buffer.get();
    buffer[index] = value;
    BCM2837_MB.mb_buffer.set(buffer);
}

#[no_mangle]
unsafe extern "C" fn set_mb_buffer(buffer: [u32; 36]) {
    BCM2837_MB.mb_buffer.set(buffer);
}

#[no_mangle]
unsafe extern "C" fn get_mb_index(index: usize) -> u32 {
    return BCM2837_MB.mb_buffer.get()[index];
}

#[no_mangle]
unsafe extern "C" fn get_mb_buffer() -> [u32; 36] {
    return BCM2837_MB.mb_buffer.get();
}

#[no_mangle]
unsafe extern "C" fn call_mb(channel: u32) -> u32 {
    compiler_fence(Ordering::Release);
    BCM2837_MB.mb_call(channel);
    0
}*/

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

pub enum MboxError {
    ResponseError,
    UnknownError,
}
pub type MbResult<T> = core::result::Result<T, MboxError>;


#[repr(C)]
#[repr(align(16))]
pub struct Bcm2837Mb {
    pub mb_buffer: [u32; 36],
}

impl ops::Deref for Bcm2837Mb {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Bcm2837Mb {
    pub fn new() -> Self {
        return Bcm2837Mb {
            mb_buffer: [0; 36],
        };
    }

    fn ptr() -> *const RegisterBlock {
        let addr = SOC_INFO.get("Peripheral") + SOC_INFO.get("GPU");
        addr as *const _
    }
}

impl KernelConsoleDriver for Bcm2837Mb {}

impl FrameBufferGraphics for Bcm2837Mb {}

impl KeyboardInput for Bcm2837Mb {}

impl Storage for Bcm2837Mb {}

impl Serial for Bcm2837Mb {}

impl Write for Bcm2837Mb {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl FirmwareInterface for Bcm2837Mb {
    fn name(&self) -> &'static str {
        self.driver_name()
    }

    fn mb_call(&self, channel: u32) -> Result<(), u32> {
        loop {
            if !self.STATUS.is_set(STATUS::FULL) {
                break;
            }
        }

        let buf_ptr = self.mb_buffer.as_ptr() as u32;

        self.WRITE.set((buf_ptr & !0xF) | (channel & 0xF));

        loop {
            loop {
                if !self.STATUS.is_set(STATUS::EMPTY) {
                    break;
                }

                unsafe { asm!("nop"); }
            }

            let resp: u32 = self.READ.get();

            if ((resp & 0xF) == channel) && ((resp & !0xF) == buf_ptr) {
                return if self.mb_buffer[1] == response::SUCCESS {
                    Ok(())
                } else if self.mb_buffer[1] == response::ERROR {
                    Err(FMI_RESPONSE_ERROR)
                } else { Err(FMI_RESPONSE_OTHER) }
            }
        }
    }

    fn status(&self) -> u32 {
        if self.STATUS.is_set(STATUS::FULL) {
            return FMI_STATUS_FULL;
        } else { return FMI_STATUS_EMPTY; }
    }

    fn add_index(&self, index: usize, val: u32) {
        // self.mb_buffer[index] = val;
    }
}

impl Led for Bcm2837Mb {}

impl Driver for Bcm2837Mb {

}
