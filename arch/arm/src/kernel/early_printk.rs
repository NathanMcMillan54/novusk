use crate::{cortex_m3};
use core::fmt::{Arguments, Result, Write};
use spin::Mutex;

lazy_static! {
    pub(crate) static ref EARLYPRINTK: Mutex<EarlyPrintk> = Mutex::new(EarlyPrintk::new());
}

pub(crate) struct EarlyPrintk {
    pub print_method: &'static str,
}

impl EarlyPrintk {
    pub fn new() -> Self {
        return EarlyPrintk { print_method: "None" };
    }

    pub fn init(&mut self, method: &'static str) {
        self.print_method = method;
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        if self.print_method == "hio" {
            cortex_m3::hio::hio_write_bytes(bytes);
        } else if self.print_method == "UART" {
            #[cfg(feature = "nrf52840")]
            nrfd::NrfUartIo::init().write_bytes(bytes);
        } else { return; }
    }

    pub fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }
}

impl Write for EarlyPrintk {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

#[export_name = "arch_printk"]
#[no_mangle]
pub extern "C" fn arm32_printk(fmt: Arguments) {
    EARLYPRINTK.lock().write_fmt(format_args!("{}{}", fmt, "\n"));
}
