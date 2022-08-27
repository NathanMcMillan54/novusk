use crate::kernel::io::types::SerialMethod;
use core::fmt::{Result, Write};

pub static mut IO: ArmIo = ArmIo::empty();

mod types {
    #[derive(Copy, Clone, PartialEq)]
    pub enum SerialMethod {
        Hio,
        Uart,
        None,
    }

    pub fn str_to_serialmethod(str: &str) -> SerialMethod {
        return match str {
            "Hio" => SerialMethod::Hio,
            "Uart" => SerialMethod::Uart,
            _ => SerialMethod::None,
        };
    }
}

pub struct ArmIo {
    pub serial_method: SerialMethod,
}

impl ArmIo {
    pub const fn empty() -> Self {
        return ArmIo { serial_method: types::SerialMethod::None };
    }

    pub fn set(&mut self, serial: &str) {
        let method = types::str_to_serialmethod(serial);

        self.serial_method = method;
    }

    pub fn write_string(&self, string: &str) {
        use types::SerialMethod;

        if self.serial_method == SerialMethod::Hio {
            self.hio_write(string);
        } else if self.serial_method == SerialMethod::Uart {
            self.uart_write(string)
        } else { return; }
    }

    pub fn hio_write(&self, string: &str) {
        #[cfg(feature = "cortex_m")]
        hprint!("{}", string);
    }

    pub fn uart_write(&self, string: &str) {
        #[cfg(feature = "rpi2")]
        crate::rpi::Rpi2::new().uart.write_str(string);
    }
}

impl Write for ArmIo {
    fn write_str(&mut self, string: &str) -> Result {
        self.write_string(string);

        Ok(())
    }
}

pub unsafe fn serial_io_init() {
    #[cfg(any(feature = "rpi2", feature = "stm32f407"))]
    IO.set("Uart");

    #[cfg(any(feature = "stellaris_6965"))]
    IO.set("Hio");
}
