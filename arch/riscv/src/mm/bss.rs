use core::mem::zeroed;
use core::ptr::write_volatile;

extern "C" {
    static mut _bss_start: u64;
    static mut _bss_end: u64;
}

pub struct BssInfo;

impl BssInfo {
    pub fn new() -> Self {
        return Self;
    }

    pub unsafe fn bss_values(&mut self) -> (u64, u64) {
        return (_bss_start, _bss_end);
    }
}
