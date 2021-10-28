use spin::Mutex;

pub mod text;

lazy_static! {
    pub static ref ARM32IO: Mutex<Arm32Io> = Mutex::new(Arm32Io::empty());
}

pub struct Arm32Io {
    pub text_method: &'static str,
    pub gpio_method: &'static str,
}

impl Arm32Io {
    pub fn empty() -> Self {
        return Arm32Io {
            text_method: "",
            gpio_method: ""
        };
    }

    pub fn init(&mut self, text: &'static str, gpio: &'static str) {
        self.text_method = text;
        self.gpio_method = gpio;
    }
}
