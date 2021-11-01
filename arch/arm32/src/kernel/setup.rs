use crate::arm32_printk;
use crate::mm::arm_mm_init;
use init::kmain;
use setup::after_kernel_setup;
use cortex_m::peripheral::Peripherals;

pub unsafe fn setup_arm32_kernel() {
    arm_mm_init();
    kinfo!("Memory initialized");
    arm32_printk!("    Allocator initialized");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    arm32_printk!("Starting after kernel...");
    after_kernel_setup();
}
