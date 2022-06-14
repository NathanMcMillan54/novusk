extern "C" {
    /// ``device_init`` is usually called in ``setup_arch``, this function sets up some device
    /// specific functions.
    pub fn device_init() -> u8;

    /// ``early_device_init`` is intended for setting device specific drivers to ``DEVICE_DRIVERS``.
    pub fn early_device_init() -> u8;
}
