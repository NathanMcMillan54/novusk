use crate::kernel::KERNEL_INFO;

extern "C" {
    fn kernel_info() -> bool;
}

pub unsafe fn early_user_init() {
    if !kernel_info() {
        KERNEL_INFO = false;
    }
}
