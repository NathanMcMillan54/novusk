pub mod boot;
pub mod cpu;
pub mod dev;
pub mod header;
pub mod main;

extern "C" {
    pub(crate) fn init();
}
