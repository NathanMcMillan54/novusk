use super::cpu::cpu_init;

#[cfg(target_arch = "x86_64")]
unsafe fn x86_64_init() {
    use crate::drivers::x64_task::x86_64_thread_init;

    //x86_64_thread_init();
    kinfo!("x86_64 thread/task initialized");
}

unsafe fn x86_init() {

}

pub unsafe fn x86_kernel_init() {
    cpu_init();
    kinfo!("CPU initialized");

    #[cfg(target_arch = "x86_64")]
    x86_64_init();
}
