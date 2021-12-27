use novusk_syscalls::SysCall;
use super::{*, sys_tbl::*};

pub unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("x86_64 Novusk System call Table");

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_read", READ, sys_read));
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", WRITE, sys_write));
    SYSCALL_TABLE.add_syscall(SysCall::new("write_init", WRITE_INIT, sys_write_init));
}
