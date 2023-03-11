use super::cpu::{cpu_init, id};
use super::interrupts::idt_init;
use super::kernel::*;
// use gpu::{GpuDrivers, GPUGRAPHICS};
use crate::boot::boot::die;
use crate::kernel::power::shutdown;
use crate::kernel::task::{Executor, Task};

unsafe fn set_drivers() {
    // When gop is supported this will change
    // GPUGRAPHICS.lock().set_driver(GpuDrivers::Vgag);
}

pub unsafe fn x86_kernel_init() {
    id::get_cpuid();
    /*KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Got cpuid",
        messages: None,
    };*/

    cpu_init();
    /*kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "CPU initialized",
        messages: Some(&[
            "GDT initialized",
            "Brand specific CPU initialized",
        ])
    });*/

    idt_init();
    /*kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Interrupts initialized",
        messages: Some(&[
            "IDT initialized",
            "Interrupts are enabled",
        ]),
    });*/

    set_drivers();
    /*kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Drivers set",
        messages: Some(&["Set GPU Graphics to VGA"]),
    });*/

    //kmain::kernel_init();
    /*kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Novusk initialized",
        messages: None,
    });*/

    //printk!("Setting up after kernel...\n");
    //after_kernel_setup();

    extern "C" { fn kernel_main(); }

    //kernel_main();
}
