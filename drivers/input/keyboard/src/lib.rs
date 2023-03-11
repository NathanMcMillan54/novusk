#![no_std]

#[macro_use] extern crate alloc;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;

pub use traits::*;

pub mod traits;
pub mod layout;
