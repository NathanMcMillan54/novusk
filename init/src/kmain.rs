use fs::{fs_init, fs_name};
use gpu::{gpu_init, gpu_name};
use input::{input_init, KEYBOARD_NAME, MOUSE_NAME};
use kernfs::kernfs_init;
use net::{net_init, net_name};
use novusk::module::{module_end, module_init};

// Modules
use m1::{m1_exit, m1_init};

#[no_mangle]
pub unsafe extern "C" fn modules_init() {
    module_init(m1_init(), "Nathan McMillan <nathanmcmillan54@gmail.com>", "m1");
    module_end(m1_exit());
}

#[no_mangle]
pub unsafe extern "C" fn mm_init() {
    allocmm::allocmm_init();
    initramfs::init_ramfs();
}

#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    modules_init();
    kinfo!("Kernel modules initialized");

    printk!("Setting up hardware input...");
    input_init();
    kinfo!("Input initialized");
    printk!("   Using {} keyboard", KEYBOARD_NAME);
    printk!("   Using {} mouse", MOUSE_NAME);

    gpu_init();
    kinfo!("GPU initialized");
    printk!("   Using {} GPU", gpu_name());

    net_init();
    kinfo!("Network drivers initialized ");
    printk!("   Using {} network driver", net_name());

    fs_init();
    kinfo!("File system initialized");
    printk!("    Using {} file system", fs_name());

    kernfs_init();
    kinfo!("Kernel File system initialized");
    printk!("   Created /temp/");

    mm_init();
    kinfo!("Memory initialized ");
    printk!("   Initialized alloc memory management");
    printk!("   Initialized initramfs (Ram File System)");
}
