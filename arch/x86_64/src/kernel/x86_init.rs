use init::kmain;
use super::cpu::{cpu_init, id};
use super::interrupts::idt_init;
use super::kernel::*;
// use gpu::{GpuDrivers, GPUGRAPHICS};
use setup::after_kernel_setup;
use crate::boot::boot::die;
use crate::kernel::power::shutdown;
use crate::kernel::task::{Executor, Task};

unsafe fn set_drivers() {
    // When gop is supported this will change
    // GPUGRAPHICS.lock().set_driver(GpuDrivers::Vgag);
}

pub unsafe fn x86_kernel_init() {
    id::get_cpuid();
    kinfo!("Got cpuid\n");
    x86_printk!("    CPU brand: {}\n", id::BRAND);

    cpu_init();
    kinfo!("CPU initialized\n");
    x86_printk!("    GDT initialized\n");
    x86_printk!("    Brand specific CPU initialized\n");

    idt_init();
    kinfo!("Interrupts initialized\n");
    x86_printk!("    IDT initialized\n");
    x86_printk!("    Interrupts are enabled\n");

    set_drivers();
    kinfo!("Drivers set\n");
    x86_printk!("    Set GPU Graphics to VGA\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    printk!("Setting up after kernel...\n");
    after_kernel_setup();

    extern "C" { fn kernel_main(); }

    kernel_main();
}
