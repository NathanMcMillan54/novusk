use novusk_syscalls::{SysCall, SysCallTable};

extern "C" {
    static mut SYSCALL_TABLE: SysCallTable;
}

pub(crate) unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("ARM32 Novusk System call Table");
}
