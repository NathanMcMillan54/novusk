use crate::define_syscall;

fn serial_write(write: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    hifive1::sprint!("{}", write);

    return sys_arg3;
}

define_syscall!(sys_write, serial_write);
