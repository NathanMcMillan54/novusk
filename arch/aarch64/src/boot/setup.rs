use core::ptr::write_volatile;
use rpi::common::{MMIO_BASE, UART_OFFSET};
use setup::{BootSetup, SetupReturn};

pub struct Aarch64Boot;

impl Aarch64Boot {
    pub fn new() -> Self {
        unsafe { write_volatile(0x3F20_100 as *mut u8, b'A'); }
        return Aarch64Boot;
    }

    pub fn setup(&self) {
        let linker = unsafe { self.linker_setup() };

        if linker.0.is_err() {
            panic!("{}", linker.1);
        }
    }
}

impl BootSetup for Aarch64Boot {
    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut __bss_start: u64;
            static mut __bss_end: u64;
        }

        r0::zero_bss(__bss_start as *mut u64, __bss_end as *mut u64);

        return (Ok(()), "Linker mem setup successfully");
    }
}
