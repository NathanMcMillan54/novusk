use super::init::KERNEL;
use super::initramfs::*;
use super::modules::modules_init;
use super::version::novusk_banner;
use kinfo::status::set_status;
use novuskinc::version::*;
use novuskinc::fs::TempFs;
use alloc::boxed::Box;

unsafe fn gpu_init() {
    KERNEL.lock().kernel_console().init();
    KERNEL.lock().gpu_graphics().init();
}

#[cfg(target_arch = "x86_64")]
unsafe fn input_init() {
    KERNEL.lock().keyboard_driver().init();
    KERNEL.lock().mouse_driver().init();
    kinfo!("Input devices initialized");
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    let mut configs = KERNEL.lock().kernel_configs();
    kinfo!("Got kernel configurations");

    // TODO: Figure out why you can't parse &str to i32 on arm
    #[cfg(not(target_arch = "arm"))]
    if configs.get("KERNEL", "MAJORVERSION").parse::<i32>().unwrap() != MAJOR_VERSION {
        panic!("Kernel config and const versions are different, the config file might be bad and unsafe");
    }

    if configs.get("GPU", "INIT") == "True" {
        gpu_init();
        kinfo!("GPU graphics initialized");
    }

    novusk_banner();

    if initramfs_type() == "Kernel" {
        kinfo!("Starting kernel initramfs...");
        start_kernel_initramfs();
    } else if initramfs_type() == "Custom" { start_custom_initramfs(); }

    #[cfg(target_arch = "x86_64")]
    input_init();

    printk!("\nSetting up main kernel modules...");
    modules_init(configs);
    kinfo!("Initialized main kernel modules");
}
