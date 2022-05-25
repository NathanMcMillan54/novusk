use core::fmt::Write;
use dif::Dif;
use novuskinc::serial::SimpleUart;

extern "C" {
    static mut DIF: Dif;
    static mut KERNEL_SIMPLEUART: SimpleUart;
}

#[no_mangle]
pub unsafe extern "C" fn early_serial_init() {
    KERNEL_SIMPLEUART.output_addr = 0x0 as *mut u8;
}
