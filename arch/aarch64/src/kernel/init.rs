use crate::aarch64_printk;
use init::init::KERNEL;
use libbmu::bmu_init;
use modules::modules::KernelModules;
use rpi::mb::MailBox;
use rpi::board::check_board;
use rpi::debug::break_point;
use core::panic::Location;

pub unsafe fn aarch64_init() {
    let mut mailbox = MailBox::new();
    mailbox.clear();
    kinfo!("Mail box cleared");

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

    KERNEL.lock().gpu_graphics().init();
    KERNEL.lock().kernel_console().init();
    kinfo!("GPU/FB graphics initialized");
    kinfo!("Kernel console initialized");

    bmu_init();
}
