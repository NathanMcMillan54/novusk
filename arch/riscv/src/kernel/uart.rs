pub trait Uart {
    fn uart_init(&mut self) {

    }

    fn write_byte(&mut self, byte: u8) {

    }

    fn write_bytes(&mut self, bytes: &[u8])  {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    fn get_byte(&mut self) -> u8 {
        return 0 as u8;
    }
}
