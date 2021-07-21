use arm::rpi::aarch64_rpi_setup;

pub unsafe fn aarch64_init() {
    aarch64_rpi_setup();
}
