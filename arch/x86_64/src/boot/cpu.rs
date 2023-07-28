use core::cell::Cell;

#[derive(Debug)]
pub struct X86_64CPU {
    pub brand: Cell<&'static str>,
    pub interrupt_handler: u8,
}

impl X86_64CPU {
    pub const fn new() -> Self {
        return X86_64CPU {
            brand: Cell::new(""),
            interrupt_handler: 0,
        }
    }
}

pub(crate) const PIC: u8 = 0;
pub(crate) const APIC: u8 = 1;
pub(crate) const X2APIC: u8 = 3;

pub(crate) static mut X86_64CPU: X86_64CPU = X86_64CPU::new();
