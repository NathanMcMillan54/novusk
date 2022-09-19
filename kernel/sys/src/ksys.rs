use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;
use super::common::*;

#[no_mangle]
pub unsafe extern "C" fn ksys_cwrite(write: *const u8, len: u8) {

}

#[no_mangle]
pub unsafe extern "C" fn ksys_write(option: u8, write: u8, len: u8) {
    match option as usize {
        KSYS_CWRITE => ksys_cwrite(write as *const u8, len),
        _ => {},
    }
}