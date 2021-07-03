use super::kernel::*;
use super::modules::x86_modules_init;
use crate::drivers::drivers::drivers_init;
use crate::drivers::ps2::keyboard::ps2_keyboard_input;


pub unsafe fn x86_kernel_init() {
    drivers_init();

    x86_modules_init();
    kinfo!("x86 modules initialized");

    ps2_keyboard_input();
}
