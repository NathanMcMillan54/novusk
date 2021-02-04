use pc_keyboard::*;
use arch::ARCH;

fn arm_keyboard() {
    kinfo!("Setting up keyboard for arm\n");
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet2, HandleControl::MapLettersToUnicode);
}

fn x86_keyboard() {
    kinfo!("Setting up keyboard for x86\n");
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
    let key_event = keyboard.add_byte(0x20);
}

pub unsafe fn keyboard_init() {
    // Imagine having keyboard support before CPU/arch support
    // pc_keyboard was intended for arm devices
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    arm_keyboard();

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    x86_keyboard();
}
