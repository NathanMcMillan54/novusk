use crate::kernel::uart::Uart;
use arm::rpi::aarch64_rpi_setup;
use libbmu::bmu_init;
use rpi::rpi_mb::clear_mailbox;

pub unsafe fn aarch64_init() {


    clear_mailbox();
    kinfo!("Mail Box cleared");

    bmu_init();
}
