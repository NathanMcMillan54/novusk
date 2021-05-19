use init::kmain::kernel_init;
use super::{KERNEL_INFO, modules};
use super::dev::{device_init, DEVICE_INFO};
use super::{printk, userspace::early};
use kerror::kerror;
use uefi_kd::screen;
use uefi::proto::console::text::{Input, Output};

#[no_mangle]
pub unsafe extern "C" fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }

    early::early_user_init();
    printk::printk_init();
    kinfo!("Kernel printing initialized");
    kinfo!("Early ueserspace initialized");

    device_init();
    kinfo!("Device initialized | running on {}", DEVICE_INFO.device_name);

    modules::modules_init();
    kinfo!("x86 modules initialized");

    kernel_init();
    kinfo!("Novusk Initialized");
}
