use core::ptr::write_volatile;

#[no_mangle]
pub unsafe extern "C" fn early_write_byte(b: u8) {
    write_volatile(0x3F20_1000 as *mut u8, b);
}
