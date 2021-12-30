pub struct GpuGraphics;

extern "C" {
    pub fn graphics_write(x: usize, y: usize, color: usize, string: &str);
    pub fn graphics_pixel(x: usize, y: usize, color: usize);
}

impl GpuGraphics {
    pub fn new() -> Self {
        return GpuGraphics;
    }

    pub fn write_string(&self, x: usize, y: usize, color: usize, string: &str) {
        unsafe { graphics_write(x, y, color, string); }
    }

    pub fn draw_pixel(&self, x: usize, y: usize, color: usize) {
        unsafe { graphics_pixel(x, y, color); }
    }
}

fn gpug_start() {
    #[cfg(target_arch = "aarch64")]
    armfb::armfb_init();

    #[cfg(target_arch = "x86_64")]
    vgag::vgag_init();
}

module_init!(gpug_init, gpug_start);

fn gpug_finish() {
    #[cfg(target_arch = "aarch64")]
    armfb::armfb_end();

    #[cfg(target_arch = "x86_64")]
    vgag::vgag_end();
}

module_end!(gpug_end, gpug_finish);
