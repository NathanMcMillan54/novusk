use init::kmain;
use super::cpu::{cpu_init, id};
use super::interrupts::idt_init;
use super::kernel::*;
use setup::{setup_kernel, after_kernel_setup};
use crate::kernel::task::executor::Executor;
use crate::kernel::task::Task;

unsafe fn set_drivers() {
    // When gop is supported this will change
    gpu::set_driver(gpu::DriverNames::Vgag);
}

pub unsafe fn x86_kernel_init() {
    id::get_cpuid();
    kinfo!("Got cpuid");
    x86_printk!("    CPU brand: {}", id::BRAND);

    cpu_init();
    kinfo!("CPU initialized");
    x86_printk!("    GDT initialized");
    x86_printk!("    Brand specific CPU initialized");

    idt_init();
    kinfo!("Interrupts initialized");
    x86_printk!("    IDT initialized");
    x86_printk!("    Interrupts are enabled");

    set_drivers();
    kinfo!("Drivers set");
    x86_printk!("    Set GPU Graphics to VGA");

    printk!("\nSetting up main kernel...");
    setup_kernel();

    kmain::kernel_init();
    kinfo!("Novusk initialized");

    printk!("Setting up after kernel...");
    after_kernel_setup();

    extern "C" { fn kernel_main(); }

    kernel_main();

    super::reboot::reboot();
}
