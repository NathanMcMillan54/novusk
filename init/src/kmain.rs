#[no_mangle]
pub unsafe extern "C" fn kernel_init() {
    printk!("Kernel init\n");
}
