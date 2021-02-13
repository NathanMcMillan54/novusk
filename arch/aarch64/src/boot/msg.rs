use crate::drivers::uart;
use core::ptr;

pub fn boot_msg(msg: &str, pos: i32) {
    for text in msg.bytes() {
        unsafe { ptr::write_volatile(uart::UART0, text) }
    }
}
