use crate::kernel::device::{device_init, device_supported};
use crate::kernel::io::ARM32IO;
use super::cpu::early_cpu_init;

pub(crate) unsafe fn early_init() {
    early_cpu_init();

    let (success, device) = device_init();

    if !device_supported(device) {
        panic!("Device is not supported");
    }

    ARM32IO.lock().init(device);

    if success.is_err() {
        panic!("{}", success.err().unwrap());
    }
}
