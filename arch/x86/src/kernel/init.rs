use kernel::info::_kinfo;
use uefi_kd::screen;
use uefi::proto::console::text::{Input, Output};
use crate::kernel::KERNEL_INFO;
use log::Level::Info;

pub unsafe fn init() {
    if !KERNEL_INFO {
        // TODO: Clear screen
        // screen::clear_screen(stdout);
    }
}
