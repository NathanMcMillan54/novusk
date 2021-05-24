#[no_mangle]
pub extern "C" fn kernel_initramfs_main() {
    // The kernel initramfs ins't going to do anything for now because there isn't a good userspace
    // yet that uses lots of memory
    return;
}
