use super::init::KERNEL;
use super::initramfs::*;
use super::modules::modules_init;
use super::version::novusk_banner;
use kinfo::status::set_status;
use novuskinc::version::*;

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
    #[cfg(all(target_arch = "x86_64", target_arch = "aarch64"))]
    start_module!(gpug_init, gpug_end);
    KERNEL.lock().kernel_console().init();
}

fn input_init() {

}

unsafe fn net_init() {
    KERNEL.lock().net_init();
    kinfo!("Network drivers initialized\n");
}

fn fs_init() {
    let mut root = KERNEL.lock().get_root_dir();

    root.root.new_dir("temp/");
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    let mut configs = KERNEL.lock().kernel_configs();
    kinfo!("Got kernel configurations\n");

    check_version(configs.get("KERNEL", "MAJORVERSION").as_str());

    printk!("Using {} config\n", configs.config_type());

    if configs.get("GPU", "INIT") == "True" {
        gpu_init();
        kinfo!("GPU graphics initialized\n");
    }

    if initramfs_type() == "Kernel" {
        kinfo!("Starting kernel initramfs...\n");
        start_kernel_initramfs();
    } else if initramfs_type() == "Custom" { start_custom_initramfs(); }

    //#[cfg(target_arch = "x86_64")]
    //input_init();

    net_init();

    // fs_init();

    printk!("\nSetting up main kernel modules...\n");
    modules_init(configs);
    kinfo!("Initialized main kernel modules\n");

    novusk_banner();
}
