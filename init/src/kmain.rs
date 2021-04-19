use super::version::{version_init};
use kernel::userspace::userspace_init;
use modules::kernelm::modules_init;

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    printk!("|------------------|\n");
    printk!("| Kernel init      |\n");
    printk!("|------------------|\n");
    // Prints version
    version_init();

    modules_init();
    info!("Kernel modules initialized\n");

    info!("Starting userspace\n");
    userspace_init();
}
