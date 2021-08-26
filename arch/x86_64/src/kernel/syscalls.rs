use super::kernel::writes;
use core::str::from_utf8_unchecked;

#[no_mangle]
pub extern "C" fn sys_write(write: u8) {
    unsafe { writes(from_utf8_unchecked(vec![write].as_slice())); }
}
