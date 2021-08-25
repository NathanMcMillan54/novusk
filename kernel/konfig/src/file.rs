use super::Konfig;

pub const DEFCONFIG: &str = include_str!("defconfig.txt");

extern "C" {
    fn custom_config() -> &'static str;
}

pub(crate) fn get_config() -> &'static str {
    #[cfg(feature = "default_config")]
    return DEFCONFIG;

    #[cfg(feature = "custom_config")]
    return unsafe { custom_config() };
}

