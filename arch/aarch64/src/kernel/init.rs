use crate::aarch64_printk;
use arm::rpi::aarch64_rpi_setup;
use libbmu::bmu_init;
use modules::modules::KernelModules;
use rpi::rpi_mb::{clear_mailbox, MAILBOX};
use rpi::board::check_board;

pub unsafe fn aarch64_init() {
    clear_mailbox();
    kinfo!("Mail Box cleared");
    aarch64_printk!("MB = {:?}", MAILBOX);

    aarch64_printk!("RPi board: {}", check_board());

    aarch64_printk!("Starting Aarch64 kernel modules...");
    modules::start::arch_modules_init(&[
        KernelModules::Ex1,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None
    ]);

    bmu_init();
}
