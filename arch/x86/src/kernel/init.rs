use init::kmain::kernel_init;
use super::{KERNEL_INFO, modules};
use super::dev::{device_init, DEVICE_INFO};
use super::io::{io_init};
use super::userspace;
use kerror::kerror;

#[no_mangle]
pub unsafe extern "C" fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }

    io_init();
    kinfo!("I/O initialized");

    device_init();
    kinfo!("Device initialized");
    printk!("   Running on {}", DEVICE_INFO.device_name);

    modules::modules_init();
    kinfo!("x86 modules initialized");

    kernel_init();
    kinfo!("Novusk Initialized");

    userspace::init::userspace_init();
    kinfo!("Userspace initialized, starting main...");
}
