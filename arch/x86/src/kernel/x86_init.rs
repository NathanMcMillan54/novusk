use super::cpu::cpu_init;

pub unsafe fn x86_init() {
    cpu_init();
    kinfo!("CPU initialized");
}
