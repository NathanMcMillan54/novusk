use core::fmt::{Arguments, Result, Write};
use core::ptr::write_volatile;

pub(crate) struct GrubVga;

impl GrubVga {
    pub fn new() -> Self {
        return GrubVga;
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe { write_volatile(0xb8000 as *mut u8, byte); }
    }

    pub fn write_string(&mut self, string: &str) {
        let bytes = string.as_bytes();

        for b in bytes {
            self.write_byte(*b);
        }
    }
}

impl Write for GrubVga {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn clear_screen() {
    let mut vga = GrubVga::new();

    for y in 0..25 {
        for x in 0..80 {
            vga.write_byte(b' ');
        }
    }
}

