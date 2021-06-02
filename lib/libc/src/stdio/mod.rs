use ctypes::{c_char, c_int};

mod stdio;

#[no_mangle]
pub unsafe extern "C" fn printf(format: *const c_char, ...) -> c_int {
    let mut fmt = format;
    stdio::vsprintf();
    return 0;
}
