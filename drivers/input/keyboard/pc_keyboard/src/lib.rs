#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate printk;

pub mod input;
pub mod layout;

pub use layout::get_pckeyboard_layout;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;

pub struct PcKeyboard;

impl PcKeyboard {
    pub fn new() -> Self {
        return PcKeyboard;
    }
}

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
