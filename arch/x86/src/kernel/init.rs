use kerror::kerror;
use uefi_kd::screen;
use uefi::proto::console::text::{Input, Output};
use super::{KERNEL_INFO, modules};

#[no_mangle]
pub unsafe extern "C" fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }



    modules::modules_init();
}
