use core::fmt::Arguments;

pub fn serial_io_write(fmt: Arguments) {

}

pub fn hio_write(fmt: Arguments) {
    #[cfg(target_arch = "arm")]
    cortex_m_semihosting::hprint!("{}", fmt);
}
