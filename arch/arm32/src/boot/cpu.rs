pub unsafe fn early_cpu_init() -> (&'static str, u32) {
    #[cfg(feature = "cortex_m")]
    return crate::cortex_m::cortex_m_init();

    #[cfg(feature = "cortex_a")]
    return crate::cortex_a::cortex_a_init();
}
