fn stellaris_irq_init() {
    // printk!("Stellaris IRQs aren't supported yet\n");
}

define_dev_irq_init!(stellaris_irq_init);
