#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate printk;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;

pub use keyboard::layout::get_pckeyboard_layout;

pub mod input;

pub struct PcKeyboard;

impl PcKeyboard {
    pub fn new() -> Self {
        return PcKeyboard;
    }
}
