use super::cpu::cpu_init;
use super::modules::x86_modules_init;

#[cfg(target_arch = "x86_64")]
unsafe fn x86_64_init() {
    use crate::drivers::x64_task::x86_64_thread_init;

    //x86_64_thread_init();
    kinfo!("x86_64 thread/task initialized");
}

pub unsafe fn x86_kernel_init() {
    cpu_init();
    kinfo!("CPU initialized");

    #[cfg(target_arch = "x86_64")]
    x86_64_init();

    x86_modules_init();
    kinfo!("x86 modules initialized");
}
