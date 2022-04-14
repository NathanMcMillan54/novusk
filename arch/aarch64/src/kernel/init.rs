use crate::early_printk;

#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_init() {
    early_printk!("Initializing Aarch64 kernel...\n");
}
