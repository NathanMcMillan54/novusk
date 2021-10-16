use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::{Stream, StreamExt};
use keyboard::{KeyboardDevice, KeyboardScancode, SCAN_CODE, WAKER};
use crate::PcKeyboard;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts::*, KeyCode};
use printk::put::puts;


impl PcKeyboard {
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
                        DecodedKey::RawKey(key) => 'K',
                    };
                }
            }
        }

        return ' ';
    }
}

pub async fn read_chars() {
    let mut scancodes = KeyboardScancode::new();
    let mut keyboard = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => puts(character.encode_utf8(&mut [0u8; 4])),
                    DecodedKey::RawKey(key) => puts("B"),
                };
            }
        }
    }
}
