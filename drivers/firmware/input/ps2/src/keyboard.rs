// Based off https://github.com/phil-opp/blog_os/blob/post-12/src/task/keyboard.rs
use core::pin::Pin;
use core::task::{Context, Poll};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::StreamExt;
use futures_util::stream::Stream;
use futures_util::task::AtomicWaker;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts};
use crate::keyboard_layout;

pub static SCAN_CODE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
pub static WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCAN_CODE.try_get() {
        if let Err(_) = queue.push(scancode) {
            printk!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        printk!("WARNING: scancode queue uninitialized");
    }
}

struct KeyboardScancode;

impl KeyboardScancode {
    pub fn new() -> Self {
        SCAN_CODE.try_init_once(|| ArrayQueue::new(9)).unwrap();
        return Self;
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

pub async fn ps2_keyboard_input() {
    let mut scancodes = KeyboardScancode::new();
    let mut keyboard = Keyboard::new(keyboard_layout(), ScancodeSet1, HandleControl::MapLettersToUnicode);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => printk!("{}", character),
                    DecodedKey::RawKey(key) => printk!("{:?}", key),
                };
            }
        }
    }
}
