use crate::{cortex_m3, cortex_m4};

pub fn device_init() {
    // Setup cpu first
    #[cfg(any(feature = "stellaris_6965"))]
    cortex_m3::cortex_m3_init();

    #[cfg(any(feature = "nrf52840", feature = "stm32f4"))]
    cortex_m4::cortex_m4_init();

    // Setup specific board
    #[cfg(feature = "stellaris_6965")]
    stellarisd::stellars_init();

    #[cfg(feature = "nrf52840")]
    nrfd::nrf_board_init();

    cfg_if! {
        if #[cfg(feature = "stm32f4")] {
            use super::early_printk::EARLYPRINTK;
            EARLYPRINTK.lock().init("hio");

            stmd::stm32f4_init();
        }
    }
}
