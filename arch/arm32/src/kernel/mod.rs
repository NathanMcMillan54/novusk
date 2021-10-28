pub mod io;
pub mod panic;
pub mod power;
pub mod setup;

extern "C" {
    pub(crate) fn device_init() -> (Result<(), &'static str>, &'static str);
}
