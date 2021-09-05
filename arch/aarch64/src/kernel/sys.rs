use super::uart::Uart;

pub(crate) fn write(sys_arg: u8) {
    let mut uart = Uart::new();

    uart.send(char::from(sys_arg));
}

pub(crate) fn read(sys_arg: u8) {
    /* let mut uart = Uart::new();
    let mut ret = 0;

    for buf in sys_arg {
        ret += uart.receive();
    }

    return ret; */
}

define_syscall!(sys_write, write);
// define_syscall!(sys_read, read);
