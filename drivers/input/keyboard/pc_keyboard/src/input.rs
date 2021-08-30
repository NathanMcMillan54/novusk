use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::{Stream, StreamExt};
use keyboard::{add_char, SCAN_CODE, WAKER, KeyboardDevice, INPUT};
use crate::PcKeyboard;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts::*};

struct KeyboardScancode;

impl KeyboardScancode {
    pub fn new() -> Self {
        SCAN_CODE.try_init_once(|| ArrayQueue::new(9)).unwrap();
        return KeyboardScancode;
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

pub fn input() -> char {
    unsafe { return INPUT; }
}

impl PcKeyboard {
    pub async fn read_char(&mut self) {
        let mut scancodes = KeyboardScancode::new();
        let mut keyboard = Keyboard::new(Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

        if let Some(scancode) = scancodes.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    return match key {
                        DecodedKey::Unicode(character) => unsafe { INPUT = character },
                        DecodedKey::RawKey(key) => unsafe { INPUT = 'A' },
                    };
                }
            }
        }
    }
}

