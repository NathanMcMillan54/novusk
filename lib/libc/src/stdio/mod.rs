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

#[no_mangle]
pub unsafe extern "C" fn scanf(format: *const c_char, ...) -> c_char {
    let mut sys = NovuskSysCalls;
    let scan = sys.sys_input().read_key();
    // TODO: Find C equivalant of Input
    return "".as_ptr() as i8;
}
