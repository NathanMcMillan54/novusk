use crate::kernel::syscalls::*;

#[no_mangle]
pub extern "C" fn syscall(sys_num: i32, sys_arg: u8) {
    match sys_num {
        1 => sys_write(sys_arg),
        _ => return,
    }
}