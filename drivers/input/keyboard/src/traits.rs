pub trait KeyboardDevice {
    fn last_ret(&mut self, ret: char) -> char {
        ret
    }

    fn read_char(&mut self) {

    }

    fn read_buf(&mut self, buf: i32) -> &[u8] {
        b""
    }
}
