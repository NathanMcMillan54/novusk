use crate::kernel::syscalls::*;

#[no_mangle]
pub unsafe extern "C" fn syscall(sys_num: i32, sys_arg: u8) -> u8 {
    match sys_num {
        63 => return read(sys_arg),
        64 => sys_write(sys_arg),
        _ => return 0,
    }

    return 0;
}
