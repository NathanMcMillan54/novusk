use super::init::KERNEL;
use super::initramfs::*;
use super::modules::modules_init;
use super::version::novusk_banner;
use kinfo::status::{set_status, KStatus};
use crate::kinfo::InfoDisplay;
use novuskinc::version::*;
use storage::storage_init;

fn check_version(version_str: &str) {
    #[cfg(not(target_arch = "arm"))]
    if version_str.parse::<i32>().unwrap() != MAJOR_VERSION {
        panic!("Kernel config and const versions are different, the config file might be bad and unsafe");
    }

    #[cfg(target_arch = "arm")]
    if version_str != "3" {
        panic!("Kernel config and const versions are different, the config file might be bad and unsafe");
    }
}

unsafe fn gpu_init() {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    start_module!(gpug_init, gpug_end);
    //KERNEL.lock().kernel_console().init();
}

fn input_init() {

}

unsafe fn net_init() {
    KERNEL.lock().net_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Network drivers initialized",
        messages: None,
    });
}

unsafe fn fs_init() {
    storage_init();

    // Call fs init function
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    let mut configs = KERNEL.lock().kernel_configs();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Go kernel configurations",
        messages: None
    });

    check_version(configs.get("KERNEL", "MAJORVERSION").as_str());

    printk!("Using {} config\n", configs.config_type());

    if configs.get("GPU", "INIT") == "True" {
        gpu_init();
        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: "GPU graphics initialized",
            messages: None
        });
    }

    if initramfs_type() == "Kernel" {
        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: "Starting kernel initramfs",
            messages: Some(&["Using default initramfs"]),
        });
        start_kernel_initramfs();
    } else if initramfs_type() == "Custom" { start_custom_initramfs(); }

    //#[cfg(target_arch = "x86_64")]
    //input_init();

    net_init();

    fs_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Fs initialized",
        messages: Some(&["Storage device initialized"]),
    });

    printk!("\nSetting up main kernel modules...\n");
    modules_init(configs);
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Initialized main kernel modules",
        messages: None
    });

    novusk_banner();
}
