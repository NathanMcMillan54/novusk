use crate::arm32_printk;
use init::kmain;
use setup::after_kernel_setup;
use spin::Mutex;
use cpu::CpuInfo;

extern "C" {
    static CPUINFO: CpuInfo;
}

pub unsafe fn test_cpu() {
    arm32_printk!("Arch: {}", CPUINFO.architecture);
}

pub unsafe fn setup_atm32_kernel() {
    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    arm32_printk!("After kernel...");
    after_kernel_setup();
}