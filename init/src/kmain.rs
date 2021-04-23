use super::version;
use include::novusk::init::{module_exit, module_init};

// Modules
use m1::{m1_exit, m1_init};

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    printk!("|------------------|\n| Kernel init      |\n|------------------|");
    version::version_init();

    kernel_modules_init();
    kinfo!("Kernel modules initialized");

    drivers::drivers_init();
    kinfo!("Main drivers initialized");

    fs::fs_init();
    kinfo!("Fs initialized");
}

unsafe fn kernel_modules_init() {
    printk!("Running kernel modules");
    printk!("   Running: {} | By: {}", m1::MODULE, m1::AUTHOR);
    module_init(m1_init(), m1::MODULE, m1::AUTHOR);
    module_exit(m1_exit());
}
