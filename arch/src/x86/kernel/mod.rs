pub mod cmdline;
pub mod early_info;
pub mod init;
pub mod vga_buffer;

extern "C" {
    pub fn time_init() -> f32;
}
