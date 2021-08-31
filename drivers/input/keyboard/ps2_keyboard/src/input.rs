use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::stream::{Stream, StreamExt};
use keyboard::{KeyboardDevice, KeyboardScancode, SCAN_CODE, WAKER};
use crate::Ps2Keyboard;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts::*, KeyCode};

impl Ps2Keyboard {
    pub fn init(&mut self) {
        let mut scancodes = KeyboardScancode::new();
        scancodes.init();
    }

    pub async fn read_char(&mut self) -> char {
        let mut scancodes = KeyboardScancode::new();
        let mut keyboard = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

        if let Some(scancode) = scancodes.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    return match key {
                        DecodedKey::Unicode(character) => character,
                        DecodedKey::RawKey(key) => '!',
                    };
                }
            }
        }

        return ' ';
    }
}
