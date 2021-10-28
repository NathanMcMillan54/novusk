pub mod io;
pub mod panic;
pub mod power;

extern "C" {
    pub(crate) fn device_init() -> (Result<(), &'static str>, &'static str);
}
