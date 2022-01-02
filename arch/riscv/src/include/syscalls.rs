use novusk_syscalls::{SysCall, SysCallTable};

extern "C" {
    pub(crate) static mut SYSCALL_TABLE: SysCallTable;
    fn sys_write(write: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
}

pub unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("RISCV Novusk System call Table");

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", 0, sys_write));

    SYSCALL_TABLE.make_call(0, b'a', 0, 0);
}
