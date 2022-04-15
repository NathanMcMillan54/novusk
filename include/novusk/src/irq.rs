extern "C" {
    /// The ``device_irq_handler`` function needs to be defined in the device driver module, it's
    /// only argument should be the IRQ number that the handler recieved. Everytime the interrupt
    /// handler gets called it will call this function to handle any device specific IRQs.
    pub fn device_irq_handler(irq: i16);
}
