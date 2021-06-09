use crate::drivers::device::DEVICE_INFO;
use init::kmain::kernel_init;
use super::modules;
use userspace::required::START_USERSPACE;
use arm_lib::include::asm::wfe;

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_init() {
    modules::modules_init();
    kinfo!("Aarch64 kernel modules initialized");

    if DEVICE_INFO.main_kernel == true {
        printk!("Starting kernel init...");
        kernel_init();
    } else if DEVICE_INFO.main_kernel == false {
        kinfo!("Aarch64 kernel initialized");
    }

    if START_USERSPACE {
        printk!("\nStarting userspace...");
        userspace::init::userspace_init();
    } else {
        printk!("\nThere is no userspace application, ending kernel...");
        printk!("   Read Documnetation/os-dev/setup.md");
        wfe()
    }
}
