#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

use novuskinc::irq::{set_irqchip, IrqChip};
use novuskinc::kernel::types::KernelFunctionName;

pub mod exceptions;

unsafe fn nvic_setup() -> u8 {
    set_irqchip(IrqChip {
        name: "NVIC",
        irq_address: 0,
        enabled: false,
        disable: empty_enable_disable,
        enable: empty_enable_disable,
        irqn: empty_irqn,
        handlers: vec![]
    });

    0
}

define_kernel_function!(KernelFunctionName::irqchip_setup, -> u8, nvic_setup);

unsafe fn nvic_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::irqchip_init, -> u8, nvic_init);

extern "C" fn empty_enable_disable() { }
extern "C" fn empty_irqn() -> i16 { 0 }