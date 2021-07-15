use core::str::from_utf8_unchecked;
use nrf52840_pac::{Peripherals, USBD};
use nrf_usbd::{Usbd, UsbPeripheral};
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use kinfo::info::set_info;

pub struct Usb;

impl Usb {
    pub fn new() -> Self {
        return Self;
    }

    pub fn read_write(&mut self) {
        let usbd = Usbd::new(Peripheral);
        let usb_bus = Usbd::new(Peripheral);
        let mut serial = SerialPort::new(&usbd);

        let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
            .product("nRF52840 Serial Port Demo")
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64)
            .build();

        unsafe {
            loop {
                if !usb_dev.poll(&mut [&mut serial]) {
                    continue;
                } else {
                    unsafe {
                        set_info("not ok");
                        kinfo!("USB device failed");
                    }
                }

                let mut buf = [0u8; 64];

                match serial.read(&mut buf) {
                    Ok(count) if count > 0 => {
                        let mut buf = &mut buf[..count];

                        printk!("Read: {:?}", from_utf8_unchecked(buf));

                        for c in buf.iter_mut() {
                            if 0x61 <= *c && *c <= 0x7a {
                                *c &= !0x20;
                            }
                        }

                        printk!("Writing: {:?}", from_utf8_unchecked(buf));
                        while !buf.is_empty() {
                            match serial.write(buf) {
                                Ok(len) => buf = &mut buf[len..],
                                _ => {}
                            }
                        }

                        printk!("Writing done");
                    }
                    _ => {}
                }

                break;
            }
        }
    }
}


struct Peripheral;

unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = USBD::ptr() as *const ();
}
