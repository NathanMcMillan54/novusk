pub fn early_cpu_init() {
    #[cfg(feature = "cortex_a")]
    crate::cortex_a::cortex_a_init();

    #[cfg(feature = "cortex_m")]
    crate::cortex_m::cortex_m_init();
}

