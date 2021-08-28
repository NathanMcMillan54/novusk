use super::uart::Uart;

pub(crate) fn write(sys_arg: u8) {
    let mut uart = Uart::new();

    uart.send(char::from(sys_arg));
}

define_syscall!(sys_write, write);
