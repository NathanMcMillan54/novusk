use arm::boot::main::arm_boot_setup;

#[entry]
unsafe fn boot() -> ! {
    arm_boot_setup();

    panic!("Stellaris kernel ended");
}
