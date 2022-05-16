#[no_mangle]
pub extern "C" fn kernel_init() {
    printk!("\nKernel init\n");
}
