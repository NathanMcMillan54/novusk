use crate::kernel::kernel::_early_write_str;
use crate::mm;
use core::fmt::Write;
use dif::DifFieldNames;
use novuskinc::prelude::early_device_init;
use novuskinc::serial::early_serial_init;
use setup::{BootSetup, SetupReturn};

pub struct ArmBootSetup;

impl ArmBootSetup {
    pub fn new() -> Self {
        return ArmBootSetup;
    }

    pub unsafe fn setup(&self) {
        unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }
        crate::liba32::libdif::set_dif();

        let linker_mem = self.linker_setup();
        unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }
        mm::early_memory_setup();
        unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }

        let early_dev = self.early_device_init();
        unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }
        let early_serial = self.early_serial_io_init();

        _early_write_str(linker_mem.1);
        _early_write_str(early_dev.1);
        _early_write_str(early_serial.1);
    }
}

impl BootSetup for ArmBootSetup {
    fn early_device_init(&self) -> SetupReturn {
        unsafe {
            if early_device_init() != 0 {
                (Err("Early device initialization failed"), "Early device initialization failed")
            } else { (Ok(()), "Initialized early device functions") }
        }
    }

    fn early_serial_io_init(&self) -> SetupReturn {
        if unsafe { early_serial_init() } == 0 {
            (Ok(()), "Finished early serial I/O setup")
        } else { (Err("Early serial I/O failed"), "Early serial I/O setup failed") }
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut __sbss: *mut u64;
            static mut __ebss: *mut u64;
        }

        r0::zero_bss(__sbss, __ebss);

        (Ok(()), "Cleared .bss section from linker memory")
    }
}