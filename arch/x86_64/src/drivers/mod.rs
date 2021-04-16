pub mod cpu;
pub mod keyboard;

pub unsafe fn early_hardware_init() {
    keyboard::keyboard_init();
    cpu::init();
}
