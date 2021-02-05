use pc_keyboard::{HandleControl, ScancodeSet2};

pub fn scancode() -> ScancodeSet2 {
    return ScancodeSet2;
}

pub fn handlecontrol() -> HandleControl {
    return HandleControl::MapLettersToUnicode;
}
