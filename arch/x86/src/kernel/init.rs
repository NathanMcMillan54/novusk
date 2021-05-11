use uefi_kd::screen;
use uefi::proto::console::text::{Input, Output};
use crate::kernel::KERNEL_INFO;

pub unsafe fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }
}
