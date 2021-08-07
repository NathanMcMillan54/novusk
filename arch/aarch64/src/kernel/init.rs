use crate::kernel::uart::Uart;
use arm::rpi::aarch64_rpi_setup;
use libbmu::bmu_init;
use core::fmt::Write;

pub unsafe fn aarch64_init() {
    vec![10; 10];

    bmu_init();
}
