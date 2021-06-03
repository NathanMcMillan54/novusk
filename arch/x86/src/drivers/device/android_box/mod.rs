use android::android_box::box_init;
use android::kernel;
use crate::drivers::device::device::Device;
use crate::kernel::cpu;
use crate::kernel::dev::setup::set_device;

pub unsafe fn android_box_init() {
    set_device(
        Device {
            cpu_brand: cpu::BRAND,
            cpu_arch: cpu::ARCH,
            device_name: "",
            device_company: ""
        }
    );
    box_init();
    kinfo!("Box initialized");
    kernel::android_init();
}
