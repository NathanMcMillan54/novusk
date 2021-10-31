use crate::kernel::io::ARM32IO;
use super::cpu::early_cpu_init;

pub(crate) fn early_init() {
    ARM32IO.lock().init("hio", "");
    unsafe { early_cpu_init(); }
}
