pub struct GpuGraphics;

impl GpuGraphics {
    pub fn new() -> Self {
        return GpuGraphics;
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
