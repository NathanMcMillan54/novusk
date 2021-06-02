use core::fmt::{Arguments, Write};
use ctypes::{c_char, c_int};
use libefi::st;

pub unsafe fn vsprintf(fmt: *const c_char) {
    let stdout = st().as_ref().stdout();
    writeln!(stdout, "{:?}", fmt);
}
