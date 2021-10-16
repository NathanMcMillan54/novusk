use crate::{GpuDrivers, GpuGraphics};

impl GpuGraphics {
    pub fn init(&mut self, size: (usize, usize)) {

    }

    pub fn uninit(&mut self) {
        if self.driver_name == GpuDrivers::Vgag {
            #[cfg(target_arch = "x86_64")]
            vgag::vgag_uninit();
        }
    }
}
