use nrf_usbd::*;
use nrf52840_pac::USBD;
use usbd::*;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid, UsbDevice};
use usbd_serial::{SerialPort, DefaultBufferStore, USB_CLASS_CDC};

pub struct NrfUsb {
    pub port: u8,
}

impl NrfUsb {
    pub fn new(usb_port: u8) -> Self {
        return NrfUsb { port: usb_port };
    }
    
    pub fn usb(&mut self) -> Usb {
        return Usb { port: self.port, disabled: false }
    }
}

impl UsbRW for NrfUsb {
    fn read(&mut self) -> u8 {
        let mut usb_data = 0;
        let usb_bus = Usbd::new(NrfUsb::new(self.port));
        let mut serial = SerialPort::new(&usb_bus);

        let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .product("nRF52840 Serial Port Demo")
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64)
            .build();

        while !self.usb().disabled {
            // If there isn't anything going in or out of the usb "disable" it
            if !usb_dev.poll(&mut [&mut serial]) {
                self.usb().disable_usb();
            }

            let mut buffer = [0u8, 64];

            match serial.read(&mut buffer) {
                Ok(rdata) if rdata > 0 => {
                    let mut data = &mut buffer[..rdata];

                    unsafe { usb_data = *data.as_ptr(); }
                }

                _ => { }
            };
        }

        return usb_data;
    }

    fn write(&mut self, bytes: &[u8]) {
        let usb_bus = Usbd::new(NrfUsb::new(self.port));
        let mut serial = SerialPort::new(&usb_bus);

        let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .product("nRF52840 Serial Port Demo")
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64)
            .build();

        while !self.usb().disabled {
            // If there isn't anything going in or out of the usb "disable" it
            if !usb_dev.poll(&mut [&mut serial]) {
                self.usb().disable_usb();
            }

            serial.write(bytes);
        }
    }
}

unsafe impl UsbPeripheral for NrfUsb {
    const REGISTERS: *const () = USBD::ptr() as *const ();
}
