use pc_keyboard::layouts::{Dvorak104Key, Uk105Key, Us104Key};

#[cfg(feature = "us_keyboard")]
pub fn keyboard_layout() -> Us104Key {
    return Us104Key;
}

#[cfg(feature = "uk_keyboard")]
pub fn keyboard_layout() -> Uk105Key {
    return Uk105Key;
}

#[cfg(feature = "custom_keyboard")]
pub fn keyboard_layout() -> Dvorak104Key {
    // If you build a keyboard you probably use dvorak
    return Dvorak104Key;
}
