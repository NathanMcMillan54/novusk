/// [``cortex_m``](https://crates.io/crates/cortex_m) and ``printk`` are needed for this file

#[no_mangle]
pub extern "C" fn specific_cpu_reboot() -> ! {
    cortex_m::peripheral::SCB::sys_reset();

    printk!("\nReboot failed\n");
    printk!("Shutting down...\n");

    specific_cpu_shutdown();
}

#[no_mangle]
pub extern "C" fn specific_cpu_shutdown() -> ! {
    panic!("Shutdown is not supported");
}