use novusk_syscalls::SysCallTable;

extern "C" {
    pub(self) static mut SYSYCALL_TABLE: SysCallTable;
}