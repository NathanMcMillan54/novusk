pub unsafe fn early_cpu_init() {
    #[cfg(feature = "cortex_m")]
    crate::cortex_m::cortex_m_init();
}
