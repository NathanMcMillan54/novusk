use core::ptr::write_volatile;

// For uart0 driver
#[no_mangle]
pub unsafe extern "C" fn early_write_byte(b: u8) {
    core::ptr::write_volatile(0x0900_0000 as *mut u8, b);
}
