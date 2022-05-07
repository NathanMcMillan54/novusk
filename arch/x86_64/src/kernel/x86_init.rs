use crate::early_printk;
use super::irq::irq_init;

unsafe fn set_drivers() {
    // When gop is supported this will change
    // GPUGRAPHICS.lock().set_driver(GpuDrivers::Vgag);
}

pub unsafe fn x86_kernel_init() {
    /*id::get_cpuid();
    kinfo!("Got cpuid\n");
    x86_printk!("    CPU brand: {}\n", id::BRAND);*/

    irq_init();
    early_printk!("IRQs initialized\n");

    /*();
    kinfo!("Drivers set\n");
    x86_printk!("    Set GPU Graphics to VGA\n");

    syscalls_init();
    kinfo!("System calls initialized\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    printk!("Setting up after kernel...\n");
    after_kernel_setup();

    extern "C" { fn kernel_main(); }

    kernel_main();*/
}
