extern "C" {
    fn kernfs_name() -> &'static str;
    fn kernfs_init();
}

pub unsafe fn kernel_fs_init() {
    printk!("Kernel file system: {}", kernfs_name());
    kernfs_init();
}
