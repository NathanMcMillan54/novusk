use uefi_kd::screen;
use uefi::proto::console::text::{Input, Output};
use super::KERNEL_INFO;

#[no_mangle]
pub unsafe extern "C" fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }

}
