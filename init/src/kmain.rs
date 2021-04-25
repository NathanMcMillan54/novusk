use super::{initramfs, noinitramfs, version};
use include::novusk::init::{module_exit, module_init};
use kernel::info::{*};

// Modules
use m1::{m1_exit, m1_init};

extern "C" { fn kernel_main() -> !; }

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    printk!("|------------------|\n| Kernel init      |\n|------------------|");
    version::version_init();

    kernel_modules_init();
    kinfo!("Kernel modules initialized");

    device_init();
    kinfo!("Device drivers initialized");

    drivers::drivers_init();
    kinfo!("Main drivers initialized");


    fs::fs_init();
    kinfo!("Fs initialized");

    initramfs_init();
    kinfo!("Initramfs initialized");

    if IS_OS == true {
        kernel_main()
    }
}

unsafe fn initramfs_init() {
    if IS_INTRAMFS == true {
        initramfs::initramfs_main();
    } else {
        noinitramfs::no_initramfs();
    }
}

unsafe fn device_init() {
    if DEVICE_NAME == "default" {
        // Nothing
    } else {
        printk!("{} - Device unsupported\n   Defaulting to 'default'", DEVICE_NAME);
        DEVICE_NAME = "default";
        device_init();
    }
}

unsafe fn kernel_modules_init() {
    printk!("Running kernel modules");
    printk!("   Running: {} | By: {}", m1::MODULE, m1::AUTHOR);
    module_init(m1_init(), m1::MODULE, m1::AUTHOR);
    module_exit(m1_exit());
}
