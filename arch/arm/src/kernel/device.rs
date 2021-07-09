use crate::nrf::nrf_init;

pub unsafe fn device_init() {
    #[cfg(feature = "nrf")]
    nrf_init();
}