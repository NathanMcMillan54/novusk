#![no_std]

#[macro_use] extern crate lazy_static;

use core::fmt::{Arguments, Result, Write};
use spin::Mutex;

pub mod printer;
pub mod setup;

use printer::empty_printk;

#[no_mangle]
pub static mut PRINTK: PrintK = PrintK {
    kernel_printer: empty_printk
};

pub struct PrintK {
    pub kernel_printer: extern "C" fn(Arguments) -> Result,
}

impl PrintK {
    pub fn new() -> Self {
        return PrintK {
            kernel_printer: empty_printk,
        };
    }
}
