pub trait Uart {
    unsafe fn uart_init(&self) {

    }

    unsafe fn uart_write_byte(&self, byte: u8) {

    }

    unsafe fn uart_write_bytes(&self, bytes: &[u8]) {
        self.uart_write_byte(*bytes.as_ptr());
    }

    unsafe fn uart_write_string(&self, sting: &str) {
        for bytes in sting.as_bytes() {
            self.uart_write_byte(*bytes);
        }
    }
}
