use crate::kernel::debug::DebugPrint;
use crate::liba64::bmu::bmu_init;
use arm::rpi::aarch64_rpi_setup;

pub unsafe fn aarch64_init() {
    let mut dprint = DebugPrint;

    dprint.write_string("[INFO] Memory initialized\n");
    dprint.write_string("    Bss start and end have been cleared\n");

    aarch64_rpi_setup();
    dprint.write_string("[INFO] Device initialized\n");
    dprint.write_string("   GPU driver set\n");

    dprint.write_string("\nStarting Mare Metal Userspace...\n");
    bmu_init();
}
