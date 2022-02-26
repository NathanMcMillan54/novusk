#![no_std]

#[macro_use] extern crate lazy_static;

use core::fmt::{Arguments, Result, Write};
use spin::Mutex;

pub mod printer;
pub mod setup;

lazy_static! {
    #[no_mangle]
    pub static ref PRINTK: Mutex<PrintK> = Mutex::new(PrintK::new());
}

pub struct PrintK {
    pub kernel_printer: extern "C" fn(Arguments) -> Result,
}

impl PrintK {
    pub fn new() -> Self {
        return PrintK {
            kernel_printer: printer::empty_printk,
        };
    }
}
