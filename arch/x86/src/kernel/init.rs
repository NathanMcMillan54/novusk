use init::kmain::kernel_init;
use super::{KERNEL_INFO, modules};
use super::dev::{device_init, DEVICE_INFO};
use super::userspace;
use kerror::kerror;
use uefi_kd::screen;
use uefi::proto::console::text::{Input, Output};

#[no_mangle]
pub unsafe extern "C" fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }

    device_init();
    kinfo!("Device initialized | running on {}", DEVICE_INFO.device_name);

    modules::modules_init();
    kinfo!("x86 modules initialized");

    kernel_init();
    kinfo!("Novusk Initialized");

    userspace::init::userspace_init();
    kinfo!("Userspace initialized, starting main...");
}
