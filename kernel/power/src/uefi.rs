use libefi::st;
use libefi::power::UefiPower;

pub unsafe fn shutdown() -> ! {
    let mut power = UefiPower;
    kinfo!("\nShutting down...");
    st().as_ref().boot_services().stall(3_000_000);

    power.shutdown();
}

pub unsafe fn reboot() -> ! {
    let mut power = UefiPower;
    kinfo!("\nRestarting...");
    st().as_ref().boot_services().stall(3_000_000);

    // I made a funny
    printk!("See you soon!");

    power.reboot();
}
