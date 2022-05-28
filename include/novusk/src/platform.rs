extern "C" {
    /// ``device_init`` is usually called in ``setup_arch``, this function sets up some device
    /// specific functions.
    pub fn device_init() -> u8;
}
