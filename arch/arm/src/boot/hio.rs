use core::fmt::{Arguments, Write};
use cortex_m_semihosting::hio::{hstdout, HStdout};

pub fn hio_write(fmt: Arguments) {
    HStdout::write_fmt(&mut hstdout().unwrap(), fmt);
}
