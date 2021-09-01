use crate::cortex_m3::cortex_m3_init;

pub fn device_init() {
    #[cfg(feature = "stellaris_6965")]
    cortex_m3_init();
}
