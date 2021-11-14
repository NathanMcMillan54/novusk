use crate::arm32_printk;
use crate::mm::arm_mm_init;
use init::kmain;
use super::{cpu, irq};
use setup::after_kernel_setup;

pub unsafe fn setup_arm32_kernel() {
    cpu::cpu_kernel_init();
    irq::irq_init();
    kinfo!("CPU initialized\n");
    arm32_printk!("    CPU info set\n");
    arm32_printk!("    Common and device IRQs initialized\n");

    arm_mm_init();
    kinfo!("Memory initialized\n");
    arm32_printk!("    Allocator initialized\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    arm32_printk!("Starting after kernel...\n");
    after_kernel_setup();
}
