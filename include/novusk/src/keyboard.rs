pub struct Keyboard<'a> {
    pub name: &'static str,
    pub input: &'a (dyn KeyboardInput + 'a),
}

impl<'a> Keyboard<'a> {
    pub const fn new(kb_name: &'static str) -> Self {
        return Keyboard {
            name: kb_name,
            input: &EmptyKeyboard as &dyn KeyboardInput,
        };
    }

    pub fn set_kb_driver(&mut self, input_driver: &'a (dyn KeyboardInput + 'a)) {
        self.input = input_driver;
    }
}

pub trait KeyboardInput {
    fn read_byte(&self) -> u8 {
        0
    }

    fn read_buf(&self, buf: u8) -> &'static [u8] {
        b""
    }

    fn interrpret_byte(&self, byte: u8) {

    }
}

struct EmptyKeyboard;

impl KeyboardInput for EmptyKeyboard {
    fn read_byte(&self) -> u8 {
        0
    }
}
