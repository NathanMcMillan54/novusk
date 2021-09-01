use cortex_m_semihosting::hio::{hstdout, HStdout};

pub fn hio_write_bytes(bytes: &[u8]) {
    HStdout::write_all(&mut hstdout().unwrap(), bytes);
}
