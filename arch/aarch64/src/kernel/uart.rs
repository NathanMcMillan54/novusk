use crate::drivers::device::DEVICE_INFO;
use core::ptr::write_volatile;

pub struct Uart0 {}

impl Uart0 {
    pub unsafe fn write_bytes(bytes: &[u8]) {
        for c in bytes {
            write_volatile(DEVICE_INFO.uart0, *c as u8);
        }
    }
}