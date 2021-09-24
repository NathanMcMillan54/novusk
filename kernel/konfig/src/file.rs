use super::Konfig;

pub const DEFCONFIG: &str = include_str!("defconfig.txt");

pub(crate) fn get_config() -> &'static str {
    extern "C" {
        fn kernel_config() -> &'static str;
    }

    unsafe { return kernel_config(); }
}

