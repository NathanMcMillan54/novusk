pub trait Uart {
    unsafe fn uart_init() {

    }

    unsafe fn uart_write_byte(byte: u8) {

    }

    unsafe fn uart_write_bytes(bytes: &[u8]) {
        Uart::uart_write_byte(*bytes.as_ptr());
    }

    unsafe fn uart_write_string(sting: &str) {
        for bytes in sting.as_bytes() {
            Uart::uart_write_byte(*bytes);
        }
    }
}