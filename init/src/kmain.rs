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

unsafe fn input_init() {
    KERNEL.lock().keyboard_driver().init();
    KERNEL.lock().mouse_driver().init();
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    let mut configs = KERNEL.lock().kernel_configs();
    kinfo!("Got kernel configurations");

    if configs.get("KERNEL", "MAJORVERSION").parse::<i32>().unwrap() != MAJOR_VERSION {
        panic!("Kernel config and const versions are different, the config file might be bad and unsafe");
    }

    if configs.get("GPU", "INIT") == "True" {
        gpu_init();
        kinfo!("GPU graphics initialized");
    }

    print_version_number();

    input_init();
    kinfo!("Input devices initialized");

    printk!("\nSetting up main kernel modules...");
    modules_init(configs);
    kinfo!("Initialized main kernel modules");
}
