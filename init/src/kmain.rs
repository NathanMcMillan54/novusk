use super::init::KERNEL;
use super::version::*;

pub fn print_version_number() {
    printk!("Running on:");
    printk!("   Novusk v{}.{}.{} {}", MAJOR_VERSION, MINOR_VERSION, REALLY_MINOR_VERSION, VERSION_NAME);
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    KERNEL.lock().gpu_graphics().init();
    KERNEL.lock().kernel_console().init();
    kinfo!("GPU graphics initialized");
    printk!("    Kernel printing re-initialized for higher screen resolution");

    print_version_number();
}
