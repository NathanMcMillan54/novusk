use super::SYSCALL_TABLE;

pub unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("x86_64 Novusk System call Table");
}