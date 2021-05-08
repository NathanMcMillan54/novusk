use super::KERNEL_INFO;

#[no_mangle]
pub unsafe extern "C" fn printk_init() -> bool {
    return KERNEL_INFO;
}
