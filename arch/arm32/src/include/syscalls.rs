use novusk_syscalls::{SysCall, SysCallTable};

pub const WRITE: u32 = 4;

extern "C" {
    static mut SYSCALL_TABLE: SysCallTable;
    fn sys_write(byte: u8, arg2: u8, arg3: u8) -> u8;
}

pub unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("ARM32 Novusk System call Table");

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", WRITE, sys_write));
}
