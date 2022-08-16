use core::borrow::BorrowMut;
use dif::Dif;
use novuskinc::prelude::*;
use crate::Device;

/// A RISCV 32/64 device struct
pub struct RiscVDevice {
    pub dif: Dif,
    pub console: Option<&'static mut dyn KernelConsoleDriver>,
}

impl Device for RiscVDevice {
    fn init(&mut self) {

    }
}
