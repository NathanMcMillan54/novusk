use crate::kernel::device::{device_init, device_supported};
use super::cpu::early_cpu_init;

pub(crate) unsafe fn early_init() {
    early_cpu_init();

    let (success, device) = device_init();
    if success.is_err() {
        panic!("{}", success.err().unwrap());
    }
}
