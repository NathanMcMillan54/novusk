pub fn device_init() {
    // Setup specific board
    #[cfg(feature = "stellaris_6965")]
    stellarisd::stellars_init();

    #[cfg(feature = "nrf52840")]
    nrfd::nrf_board_init();

    #[cfg(any(feature = "stm32f401", feature = "stm32f407"))]
    stmd::stm32f4_init();
}
