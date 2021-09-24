use crate::mm::ALLOCATOR;
use init::kmain;
use super::cpu::cpu_init;
use super::early_printk::EARLYPRINTK;
use libbmu::bmu_init;

pub unsafe fn setup_kernel() {
    printk!("Setting up kernel...");

    cpu_init();
    kinfo!("CPU initialized");
    printk!("   IRQ interrupts initialized");

    kmain::kernel_init();
    printk!("\nNovusk initialized");

    kinfo!("Memory used: {}B", ALLOCATOR.used());

    ALLOCATOR.free();
    kinfo!("Memory freed");

    bmu_init();
}
