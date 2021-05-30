use libefi::st;
use libnu::ktypes::ApplicationType;
use super::required::APPLICATION_TYPE;

extern "C" {
    fn kernel_main();
}

#[no_mangle]
pub unsafe extern "C" fn userspace_init() {
    if APPLICATION_TYPE == ApplicationType::OperatingSystem {
        st().as_ref().stdout().clear();
        // In the future fill screen with MAIN_COLOR
    } else if APPLICATION_TYPE == ApplicationType::KernelExtension {
        printk!("\nStarting kernel extenstion...\n");
    } else {
        printk!("Application type is unknown starting\n    Starting main anyway...");
    }

    kernel_main();
}
