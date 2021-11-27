use crate::kernel::setup::setup_arm32_kernel;

pub fn init_arm_kernel() {
    unsafe { setup_arm32_kernel(); }
}
