use arm::boot::main::arm_boot_setup;
use novuskinc::kernel::start_kernel;

#[entry]
unsafe fn boot() -> ! {
    arm_boot_setup();

    start_kernel();

    panic!("Stellaris kernel ended");
}
