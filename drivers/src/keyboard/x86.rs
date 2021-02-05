use pc_keyboard::{HandleControl, ScancodeSet1};

pub fn scancode() -> ScancodeSet1 {
    return ScancodeSet1;
}

pub fn handlecontrol() -> HandleControl {
    return HandleControl::Ignore;
}
