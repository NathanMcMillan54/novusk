extern "C" {
    /// ``device_timer_init`` is used to initialize the device's timer peripheral.
    ///
    /// ```rust
    /// unsafe fn timer_init(_n: ()) -> () {
    ///     ...
    ///     return ();
    /// }
    ///
    /// define_core_function!(CoreFunctionNames::)
    /// ```
    pub fn device_timer_init();
}
