use uefi_kd::screen;
use crate::kernel::KERNEL_INFO;
use uefi::proto::console::text::{Input, Output};

pub unsafe fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }
}
