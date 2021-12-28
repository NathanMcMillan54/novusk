use novuskinc::kernel::syscalls::table::SYSCALL_TABLE;

pub(crate) unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("RISCV Novusk System call Table");
}