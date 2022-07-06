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
    /// define_kernel_function!(KernelFunctionName::device_irq_handler, irqn: i16, -> (), irq_handler);
    /// ```
    pub fn device_irq_handler(irq: i16);

    /// ``device_specific_irqs_init`` is used for initializing IRQs that are specific to the device
    /// Novusk is running on.
    pub fn device_specific_irqs_init();

    /// ``irqchip_setup`` is used to start setting up the IRQ chip before it's initialized, it
    /// should get called in ``setup_arch``. Sometimes it will add some IRQ handlers to "test" them.
    pub fn irqchip_setup();

    /// ``irqchip_init`` is used to initialize the device's IRQ chip. The IRQ chip is what controls
    /// the IRQs, this function needs to be implemented in the IRQ chip driver.
    pub fn irqchip_init();

    /// The ``notify_irq`` function needs to be implemented to tell the IRQ chip that an IRQ has
    /// finished
    pub fn notify_irq(irqn: u8);
}
