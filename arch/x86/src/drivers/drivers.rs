use crate::kernel::kernel::*;

pub(crate) trait Drivers {
    unsafe fn cpu_driver_init(&self) {

    }

    unsafe fn drivers_main(&self) {
        self.cpu_driver_init();
        kinfo!("Drivers initialized");
        x86_printk!("    CPU drivers initialized");
    }
}

pub unsafe fn drivers_init() {
    #[cfg(target_arch = "x86_64")]
    super::x64::x64_drivers_init();
}
