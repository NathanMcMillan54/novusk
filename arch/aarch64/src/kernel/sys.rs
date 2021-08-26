use super::uart::Uart;

#[no_mangle]
pub extern "C" fn sys_write(sys_arg: u8) {
    let mut uart = Uart::new();

    uart.send(char::from(sys_arg));
}
