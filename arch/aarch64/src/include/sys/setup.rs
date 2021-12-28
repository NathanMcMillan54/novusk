use novusk_syscalls::{SysCall, SysCallTable};
use super::systbl::*;

extern "C" {
    static mut SYSCALL_TABLE: SysCallTable;
    fn sys_write(write_byte: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    fn sys_read(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
}

pub unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("Aarch64 Novusk System call Table");

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", WRITE, sys_write));
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_read", READ, sys_read));
}
