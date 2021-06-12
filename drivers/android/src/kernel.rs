use crate::color::switch_color;
use libnu::types::ApplicationType;
use uefi::proto::console::text::Color;
use net::{net_init, net_name};

pub unsafe fn android_init() -> ! {
    printk!("Starting Android kernel...");

    // Switch back to originial color because too much green text looks weird
    switch_color(Color::LightGray, Color::Black);
    fs::fs_init();
    switch_color(Color::LightGreen, Color::Black);
    kinfo!("File system initialized");

    net_init();
    kinfo!("Network drivers initialized");
    printk!("   Using {} driver", net_name());

    #[cfg(target_arch = "x86_64")]
    allocmm::allocmm_init();

    initramfs::init_ramfs();
    kinfo!("Memory initialized");
    printk!("   Alloc MM initialized");
    printk!("   Ram Fs initialized");

    kinfo!("Novusk Android kernel initialized");

    // Application type is KernelExtension because it will start another kernel (the Android OS kernel)
    userspace::required::set_userspace_info(ApplicationType::KernelExtension, "green");
    kinfo!("Userspace initialized");
    printk!("Starting Android...");
    switch_color(Color::LightGray, Color::Black);
    userspace::init::userspace_init();
    loop {  }
}
