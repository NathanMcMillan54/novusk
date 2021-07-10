use crate::nrf::nrf_init;

pub enum Board {
    Nrf52840,
    Nrf52832,
}

pub unsafe fn device_init() {
    #[cfg(feature = "nrf")]
    nrf_init();
}