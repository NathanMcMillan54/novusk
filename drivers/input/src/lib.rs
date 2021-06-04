#![no_std]

pub static mut KEYBOARD_NAME: &'static str = "Unknown";
pub static mut MOUSE_NAME: &'static str = "Unknown";

#[cfg(target_arch = "x86_64")]
unsafe fn x64_input_init() {
    x64_ps2_mouse::init::init_mouse();
    MOUSE_NAME = "ps2";
}

pub unsafe fn input_init() {
    #[cfg(target_arch = "x86_64")]
    x64_input_init();

    // Supported on most architectures
    /* TODO: In the future when there is an input device that is definitely supported on UEFI
        (like USB) make setup done by features */
    ps2_keyboard::init::ps2_keyboard_init();
    KEYBOARD_NAME = "ps2";
}
