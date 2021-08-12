use core::ptr::{write_volatile};

pub struct Usb {
    pub port: u8,
    disabled: bool,
}

impl Usb {
    pub fn new(port_address: u8) -> Self {
        return Usb {
            port: port_address,
            disabled: false,
        };
    }

    pub fn disable_usb(&mut self) {
        self.disabled = true;
    }
}
