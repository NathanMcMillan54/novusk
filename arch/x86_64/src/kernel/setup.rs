use alloc::string::ToString;
use core::panic::PanicInfo;
//use novuskinc::irq::irqchip_init;
use archinc::{disable_irqs, enable_irqs};
use crate::kernel::gdt::gdt_init;
use crate::kernel::idt::idt_init;

unsafe fn setup_irqs() {
    /*disable_irqs();

    gdt_init();
    idt_init();

    enable_irqs()*/
}

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    /*setup_irqs();
    //setup_irqs();
    early_printk!("GDT and IDT initialized\n");

    irqchip_init();
    early_printk!("IRQ chip initialized\n");
    x86_64::instructions::interrupts::enable();
    early_printk!("Interrupts enabled\n");*/
}
