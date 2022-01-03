pub mod bases {
    #[cfg(not(any(feature = "rpi3", feature = "rpi2")))]
    pub const MMIO_BASE: u32 = 0;

    #[cfg(any(feature = "rpi3", feature = "rpi2"))]
    pub const MMIO_BASE: u32 = 0x3F00_0000;

    // This is common for all boards
    pub const GPIO_BASE: u32 = MMIO_BASE + 0x0020_0000;
    pub const UART_OFFSET: u32 = 0x0020_1000;
    pub const PL011_UART_START: u32 = MMIO_BASE + UART_OFFSET;
}

//#[cfg(any(feature = "rpi2", feature = "rpi3"))]
pub mod rpi_2_3_mb {
    use core::ops;
    use tock_registers::register_bitfields;
    use tock_registers::registers::{ReadOnly, WriteOnly};
    use tock_registers::interfaces::{Readable, Writeable};
    use mailbox::MailBox;
    use crate::MMIO_BASE;

    register_bitfields! {
        u32,

        STATUS [
            FULL  OFFSET(31) NUMBITS(1) [],
            EMPTY OFFSET(30) NUMBITS(1) []
        ]
    }

    const VIDEOCORE_MBOX: u32 = MMIO_BASE + 0xB880;

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
    pub struct RpiMb {
        pub mb_buffer: [u32; 36],
    }

    impl ops::Deref for RpiMb {
        type Target = RegisterBlock;

        fn deref(&self) -> &Self::Target {
            unsafe { &*Self::ptr() }
        }
    }

    impl RpiMb {
        pub fn new() -> Self {
            RpiMb { mb_buffer: [0; 36] }
        }

        fn ptr() -> *const RegisterBlock {
            VIDEOCORE_MBOX as *const _
        }

        pub fn init(&self) {

        }

        pub fn call(&self, channel: u32) -> MbResult<()> {
            loop {
                if !self.STATUS.is_set(STATUS::FULL) {
                    break;
                }

                unsafe { llvm_asm!("nop" :::: "volatile") };
            }

            let buf_ptr = self.mb_buffer.as_ptr() as u32;

            self.WRITE.set((buf_ptr & !0xF) | (channel & 0xF));

            loop {
                loop {
                    if !self.STATUS.is_set(STATUS::EMPTY) {
                        break;
                    }

                    unsafe { llvm_asm!("nop" :::: "volatile") };
                }

                let resp: u32 = self.READ.get();

                if ((resp & 0xF) == channel) && ((resp & !0xF) == buf_ptr) {
                    return match self.mb_buffer[1] {
                        response::SUCCESS => Ok(()),
                        response::ERROR => Err(MboxError::ResponseError),
                        _ => Err(MboxError::UnknownError),
                    };
                }
            }
        }
    }
}

pub use bases::*;

pub use rpi_2_3_mb::*;
