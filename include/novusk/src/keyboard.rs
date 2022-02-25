pub struct Keyboard<'a> {
    pub name: &'static str,
    pub input: &'a (dyn KeyboardInput + 'a),
}

impl<'a> Keyboard<'a> {
    pub fn new(kb_name: &'static str) -> Self {
        return Keyboard {
            name: kb_name,
            input: EmptyKeyboard,
        };
    }
}

pub trait KeyboardInput {
    fn read_byte() -> u8 {
        0
    }

    fn read_buf(buf: u8) -> &[u8] {
        b""
    }
}

struct EmptyKeyboard;

impl KeyboardInput {

}
