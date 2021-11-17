#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate printk;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;

pub mod input;

pub use pc_keyboard::{DecodedKey, KeyCode, KeyEvent, KeyState};

pub struct PcKeyboard;

impl PcKeyboard {
    pub fn new() -> Self {
        return PcKeyboard;
    }
}
