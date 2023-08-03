use novuskinc::irq::irqchip_init;
use crate::early_printk;
use crate::kernel::gdt::gdt_init;
use crate::kernel::idt::idt_init;

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    gdt_init();
    early_printk!("Initialized GDT\n");

    idt_init();
    early_printk!("Initialized IDT\n");

    irqchip_init();
    early_printk!("IRQ chip initialized\n");
    x86_64::instructions::interrupts::enable();
    early_printk!("Interrupts enabled\n");
}
