use crate::kernel::device::SUPPORTED_DEVICES;
use spin::Mutex;

pub mod serial;
pub mod types;
use types::{str_to_serialmethods, SerialMethods};

lazy_static! {
    pub static ref ARM32IO: Mutex<Arm32Io> = Mutex::new(Arm32Io::empty());
}

#[derive(Copy, Clone)]
pub struct Arm32Io {
    serial_method: SerialMethods,
    pub device: &'static str,
}

impl Arm32Io {
    pub fn empty() -> Self {
        return Arm32Io {
            serial_method: SerialMethods::default(),
            device: "",
        };
    }

    fn check_serial_method(self, dev_name: &str) -> &str {
        if dev_name == SUPPORTED_DEVICES[1] {
            return "Hio";
        } else { return "UART"; }
    }

    pub fn init(&mut self, device: &'static str) {
        self.serial_method = str_to_serialmethods(self.check_serial_method(device));
        self.device = device;
    }
}
