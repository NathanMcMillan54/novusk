use crate::kernel::debug::DebugPrint;
use arm::rpi::aarch64_rpi_setup;
use libbmu::bmu_init;

pub unsafe fn aarch64_init() {
    let mut dprint = DebugPrint;

    dprint.write_string("[INFO] Memory initialized\n");
    dprint.write_string("    Bss start and end have been cleared\n");

    dprint.write_string("[INFO] Device initialized\n");
    dprint.write_string("   GPU driver set\n");

    dprint.write_string("\nStarting Bare Metal Userspace...\n");
    bmu_init();
}
