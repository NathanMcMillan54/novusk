use pc_keyboard::{HandleControl, Keyboard, ScancodeSet1};
use pc_keyboard::layouts::*;

fn check_us_layout() -> bool {
    let mut kb = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

    return match kb.add_byte(0x20) {
        Ok(Some(K)) => true,
        Ok(None) => false,
        Err(E) => false,
    };
}

fn check_uk_keyboard() -> bool {
    let mut kb = Keyboard::new(Uk105Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

    return match kb.add_byte(0x20) {
        Ok(Some(K)) => true,
        Ok(None) => false,
        Err(E) => false,
    };
}

fn check_jis_keyboard() -> bool {
    let mut kb = Keyboard::new(Jis109Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

    return match kb.add_byte(0x20) {
        Ok(Some(K)) => true,
        Ok(None) => false,
        Err(E) => false,
    };
}

fn check_azerty_keyboard() -> bool {
    let mut kb = Keyboard::new(Azerty, ScancodeSet1, HandleControl::MapLettersToUnicode);

    return match kb.add_byte(0x20) {
        Ok(Some(K)) => true,
        Ok(None) => false,
        Err(E) => false,
    };
}

fn check_dvorak_keyboard() -> bool {
    let mut kb = Keyboard::new(Dvorak104Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

    return match kb.add_byte(0x20) {
        Ok(Some(K)) => true,
        Ok(None) => false,
        Err(E) => false,
    };
}

pub fn get_pckeyboard_layout() -> &'static str {
    if check_us_layout() {
        "Us"
    } else if check_uk_keyboard() {
        "Uk"
    } else if check_jis_keyboard() {
        "Jis"
    } else if check_azerty_keyboard() {
        "Azerty"
    } else if check_dvorak_keyboard() {
        "Dvorak"
    } else { "Unknown" }
}
