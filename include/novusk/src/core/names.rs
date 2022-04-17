/// The ``CoreFunctionNames`` enum is used for defining core kernel functions.
#[allow(non_snake_case)]
#[derive(Copy, Clone, PartialEq)]
pub enum CoreFunctionNames {
    /// This is an empty function that does nothing, it's just an empty function.
    empty,

    /// This function is used for initializing device specific IRQs. If a device has it's own IRQs
    /// that won't be handled by the kernel, this function can be defined in a device kernel module
    /// to initialize those IRQs.
    device_specific_irqs_init,

    /// This function is used to handle device specific IRQs that the kernel doesn't handle. It
    /// should have one argument with an ``i16`` type, this argument is the IRQ number that was
    /// given.
    device_irq_handler,

    /// This function is for initializing the device timer. It shouldn't take any arguments.
    device_timer_init,
}
