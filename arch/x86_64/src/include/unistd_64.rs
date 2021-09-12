pub fn syscall(sys_num: i32, sys_arg: u8) -> u8 {
    crate::kernel::syscalls::sys_write(sys_arg);
    return 0;
}
