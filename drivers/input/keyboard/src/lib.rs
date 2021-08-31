#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate printk;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;

pub use scancode::*;
pub use traits::*;

pub mod scancode;
pub mod traits;
pub mod layout;

pub static SCAN_CODE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
pub static WAKER: AtomicWaker = AtomicWaker::new();

pub fn add_char(add: char) {
    //unsafe { INPUT.push(add); }
}

pub unsafe fn check_finished_input() -> bool {
    /*let input_len = INPUT.len();

    for i in 0..input_len {
        if INPUT[i] == '\n' || INPUT[i] == '\r' {
            return true;
        }
    }
*/
    return false;
}

pub unsafe fn get_input(index: usize) -> char {
    return 'a';
}

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
