extern "C" {
    /// The ``device_irq_handler`` function needs to be defined in the device kernel module, it's
    /// only argument should be the IRQ number that the handler received. Everytime the interrupt
    /// handler gets called it will call this function to handle any device specific IRQs.
    ///
    /// Defining:
    /// ```rust
    /// unsafe fn irq_handler(irqn: i16) {
    ///     match irqn {
    ///         n => ...,
    ///         _ => return,
    ///     }
    /// }
    ///
    /// define_core_function!(CoreFunctionNames::device_irq_handler, irqn: i16, -> (), irq_handler);
    /// ```
    pub fn device_irq_handler(irq: i16);

    pub fn device_specific_irqs_init();
}
