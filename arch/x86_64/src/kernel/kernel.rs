pub use super::gop::init::gop_init;
pub use super::vga::init::vga_init;

extern "C" {
    pub fn check_cpuid() -> i32;
    pub fn set_CPU_BRAND();
    pub static mut CPU_BRAND: *const u8;
}
