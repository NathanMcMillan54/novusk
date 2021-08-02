// Based off https://github.com/phil-opp/blog_os/blob/post-12/src/task/keyboard.rs
use core::pin::Pin;
use core::task::{Context, Poll};
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::StreamExt;
use futures_util::stream::Stream;
use futures_util::task::AtomicWaker;
use crate::drivers::ps2::tests::KEYBOARD_PASSED;
use libcolor::vga_colors::Color;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts};
// use crate::drivers::vga::pixel::_pixel;
use crate::x86_printk;
use crate::drivers::ps2::keyboard::setup::keyboard_layout;
use crate::include::other::pixel::_vga_pixel;

pub static SCAN_CODE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
pub static WAKER: AtomicWaker = AtomicWaker::new();

struct KeyboardScancode;

impl KeyboardScancode {
    pub fn new() -> Self {
        SCAN_CODE.try_init_once(|| ArrayQueue::new(100)).unwrap();
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

pub async unsafe fn ps2_keyboard_input() {
    if !KEYBOARD_PASSED {
        _vga_pixel(Color::Yellow, 0, 0);
    }

    let mut scancodes = KeyboardScancode::new();
    let mut keyboard = Keyboard::new(keyboard_layout(), ScancodeSet1, HandleControl::MapLettersToUnicode);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => x86_printk!("{}", character),
                    DecodedKey::RawKey(key) => x86_printk!("{:?}", key),
                }
            }
        }
    }
}
