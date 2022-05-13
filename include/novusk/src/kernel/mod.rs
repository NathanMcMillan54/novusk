extern "C" {
    /// This function is used to setup the architecture specific kernel
    pub fn setup_arch();

    /// ``arch_prepare_init`` uses the architecture specific kernel to get ready for the main kernel
    /// initialization
    pub fn arch_prepare_init();
}
