use asminc::aarch64::io::outb;
use super::timer::SysTimer1;
use crate::SOC_INFO;

pub mod irqns {
    pub const SYS_TIMER_0: isize = 1 << 0;
    pub const SYS_TIMER_1: isize = 1 << 1;
}

pub const IRQ_ENABLE_1: isize = 0xB210;
const RPI3_IRQS: &[unsafe extern "C" fn(); 1] = &[SysTimer1];

pub unsafe fn rpi3_enable_irqs() {
    outb(0x3F00_0000 + IRQ_ENABLE_1 as u32, irqns::SYS_TIMER_1 as u16);
}

pub unsafe fn rpi3_irq_handler(irqn: i16) {
    match irqn as isize {
        irqns::SYS_TIMER_1 => super::timer::SysTimer1(),

        _ => return,
    }
}
