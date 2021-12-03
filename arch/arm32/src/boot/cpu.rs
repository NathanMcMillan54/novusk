pub fn early_cpu_init() {
    #[cfg(feature = "cortex_a")]
    crate::target::cortex_a_init();

    #[cfg(feature = "cortex_m")]
    crate::target::cortex_m_init();
}

