use crate::{cortex_m3, cortex_m4};

pub fn device_init() {
    // Setup specific board
    #[cfg(feature = "stellaris_6965")]
    stellarisd::stellars_init();

    #[cfg(feature = "nrf52840")]
    nrfd::nrf_board_init();


}
