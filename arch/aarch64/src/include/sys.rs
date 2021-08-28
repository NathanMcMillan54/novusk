use crate::kernel::sys::*;

#[no_mangle]
pub extern "C" fn syscall(sys_num: i32, sys_arg: u8) -> u8 {
    match sys_num {
        4 => sys_write(sys_arg),
        _ => return 0,
    }

    return 0;
}
