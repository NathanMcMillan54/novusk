use super::init::KERNEL;
use super::modules::modules_init;
use super::version::*;
use kinfo::status::set_status;

pub fn print_version_number() {
    printk!("Running on:");
    printk!("    Novusk v{}.{}.{} {}", MAJOR_VERSION, MINOR_VERSION, REALLY_MINOR_VERSION, VERSION_NAME);
}

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

    print_version_number();

    #[cfg(target_arch = "x86_64")]
    input_init();

    printk!("\nSetting up main kernel modules...");
    modules_init(configs);
    kinfo!("Initialized main kernel modules");
}
