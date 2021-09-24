use time::{cpu, kernel};

pub(crate) fn interrupt0() {
    unsafe {
        cpu::CPU_TIME += 1;
        kernel::update_kernel_time(1);
    }
}
