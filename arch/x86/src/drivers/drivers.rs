use crate::kernel::kernel::*;

pub(crate) trait Drivers {
    unsafe fn cpu_driver_init(&self) {

    }

    unsafe fn hardware_input_init(&self) {

    }

    unsafe fn drivers_init(&self) {
        self.cpu_driver_init();
        self.hardware_input_init();
        kinfo!("Drivers initialized");
        x86_printk!("    CPU drivers initialized");
        x86_printk!("    Hardware input initialized (keyboard/mouse)");
    }
}

pub unsafe fn drivers_init() {
    #[cfg(target_arch = "x86_64")]
    super::x64::x64_drivers_init();

    #[cfg(target_arch = "x86")]
    super::x86::x86_drivers_init();
}
