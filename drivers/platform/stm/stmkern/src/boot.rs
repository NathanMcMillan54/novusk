use arm::boot::main::arm_boot_setup;
use cortex_m_rt::entry;
use novuskinc::kernel::start_kernel;

#[entry]
unsafe fn boot() -> ! {
    arm_boot_setup();

    start_kernel();

    panic!("STM kernel ended");
}
