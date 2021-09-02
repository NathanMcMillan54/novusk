use crate::mm::ALLOCATOR;
use init::kmain;
use super::early_printk::EARLYPRINTK;
use libbmu::bmu_init;

pub unsafe fn setup_kernel() {
    EARLYPRINTK.lock().write_bytes(b"Setting up kernel...\n");

    kmain::kernel_init();
    printk!("\nNovusk initialized");

    kinfo!("Memory used: {}B", ALLOCATOR.used());

    ALLOCATOR.free();
    kinfo!("Memory freed");

    bmu_init();
}
