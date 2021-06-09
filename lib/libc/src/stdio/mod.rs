use ctypes::{c_char, c_int};
use novusk_syscalls::NovuskSysCalls;
use core::any::Any;

mod stdio;

#[no_mangle]
pub unsafe extern "C" fn printf(format: *const c_char, ...) -> c_int {
    let mut fmt = format;
    stdio::vsprintf(fmt);
    return 0;
}

