use spin::Mutex;

pub mod text;
pub mod types;
use types::{str_to_textmethods, TextMethods};

lazy_static! {
    pub static ref ARM32IO: Mutex<Arm32Io> = Mutex::new(Arm32Io::empty());
}

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

    pub fn init(&mut self, device: &'static str) {
        self.text_method = str_to_textmethods(device);
        self.device = device;
    }
}
