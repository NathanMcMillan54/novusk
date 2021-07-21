// For debug printing in Qemu
use core::fmt::{Result, Write};
use core::ptr::write_volatile;

pub struct DebugPrint;

impl DebugPrint {
    pub fn write_string(&mut self, string: &str) {
        for c in string.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
    }
}

impl Write for DebugPrint {
    fn write_str(&mut self, str: &str) -> Result {
        unsafe { self.write_string(str); }
        Ok(())
    }
}

pub fn console() -> impl Write {
    return DebugPrint;
}
