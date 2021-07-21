use crate::kernel::debug::DebugPrint;
use arm::rpi::aarch64_rpi_setup;

pub unsafe fn aarch64_init() {
    let mut dprint = DebugPrint;

    aarch64_rpi_setup();
    dprint.write_string("[INFO] Device initialized\n");
    dprint.write_string("   GPU driver set\n");
}
