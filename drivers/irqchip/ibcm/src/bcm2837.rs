use core::fmt::Arguments;
use asminc::aarch64::io::{inb, outb};
use crate::SOC_INFO;

pub struct Bcm2837IrqChip;

pub static mut BCM2837_IRQCHIP: Bcm2837IrqChip = Bcm2837IrqChip;

extern "C" {
    fn _early64_printk(Arguments);
}

#[no_mangle]
pub unsafe extern "C" fn irqchip_init() {
    /*let irq_base = SOC_INFO.get("IRQ Base Address");

    if irq_base.is_none() {
        panic!("Can't find IRQ Base Address, cannot setup BCM2837 irq chip");
    }*/

    _early64_printk(format_args!("{}", "Initializing..."));

    outb(0x3F00_0000 + 0xB210, 1 << 1 as u16);

    _early64_printk(format_args!("{}", "Initialized\n"));
}

pub mod irqs {
    pub const TIMER1: u8 = 1;
    pub const TIMER3: u8 = 3;
    pub const USB: u8 = 9;
    pub const AUX: u8 = 29;
    pub const GPIO0: u8 = 49;
    pub const GPIO1: u8 = 50;
    pub const GPIO2: u8 = 51;
    pub const GPIO3: u8 = 52;
    pub const UART: u8 = 57;
}
