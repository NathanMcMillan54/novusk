use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::{Stream, StreamExt};
use keyboard::{KeyboardDevice, INPUT, SCAN_CODE, WAKER};
use crate::PcKeyboard;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts::*, KeyCode};
use printk::put::puts;

struct KeyboardScancode;

impl KeyboardScancode {
    pub fn new() -> Self {
        return KeyboardScancode;
    }

    pub fn init(&mut self) {
        SCAN_CODE.try_init_once(|| ArrayQueue::new(9)).unwrap();
    }
}

impl Stream for KeyboardScancode {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCAN_CODE.try_get().unwrap();

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

fn char_input(input: char) -> char {
    printk!("char input: {}", input);

    return input;
}

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
                        DecodedKey::Unicode(character) => char_input(character),
                        DecodedKey::RawKey(key) => 'B',
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
