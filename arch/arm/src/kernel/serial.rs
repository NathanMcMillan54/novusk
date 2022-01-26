use core::fmt::Arguments;

pub fn serial_io_write(fmt: Arguments) {

}

pub fn hio_write(fmt: Arguments) {
    cortex_m_semihosting::hprint!("{}", fmt);
}
