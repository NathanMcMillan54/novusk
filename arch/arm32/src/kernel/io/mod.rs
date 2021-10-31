use crate::kernel::device::SUPPORTED_DEVICES;
use spin::Mutex;

pub mod text;
pub mod types;
use types::{str_to_textmethods, TextMethods};

lazy_static! {
    pub static ref ARM32IO: Mutex<Arm32Io> = Mutex::new(Arm32Io::empty());
}

#[derive(Copy, Clone)]
pub struct Arm32Io {
    text_method: TextMethods,
    pub device: &'static str,
}

impl Arm32Io {
    pub fn empty() -> Self {
        return Arm32Io {
            text_method: TextMethods::default(),
            device: "",
        };
    }

    fn check_text_method(self, dev_name: &str) -> &str {
        if dev_name == SUPPORTED_DEVICES[1] {
            return "Hio";
        } else { return "UART"; }
    }

    pub fn init(&mut self, device: &'static str) {
        self.text_method = str_to_textmethods(self.check_text_method(device));
        self.device = device;
    }
}
