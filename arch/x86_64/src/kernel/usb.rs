use usbd::{UsbRW, Usb};
use x86::io::{inb, outb};

pub struct X64Usb {
    pub port: u8,
}

impl X64Usb {
    pub fn new(usb_port: u8) -> Self {
        return X64Usb { port: usb_port };
    }

    pub fn usb(&mut self) -> Usb {
        return Usb::new(self.port);
    }
}

impl UsbRW for X64Usb {
    fn read(&mut self) -> u8 {
        let mut usb_input = 0;

        while !self.usb().disabled {
            unsafe { usb_input = inb(self.port as u16) };
        }

        return usb_input;
    }

    fn write(&mut self, byte: &[u8]) {
        unsafe {
            while !self.usb().disabled {
                outb(self.port as u16, *byte.as_ptr());
            }
        }
    }
}
