use crate::{cortex_m3, cortex_m4};

pub fn device_init() {
    // Setup cpu first
    #[cfg(any(feature = "stellaris_6965"))]
    cortex_m3::cortex_m3_init();

    #[cfg(any(feature = "nrf52840"))]
    cortex_m4::cortex_m4_init();

    // Setup specific board
    #[cfg(feature = "nrf52840")]
    nrfd::nrf_board_init();
}
