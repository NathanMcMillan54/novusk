use crate::kernel::syscalls::*;

pub fn syscall(sys_num: i32, sys_arg: u8) {
    match sys_num {
        1 => sys_write(sys_arg),
        _ => return,
    }
}