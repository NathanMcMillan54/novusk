use super::{cmdline, cpu};
use crate::akernel::{init, kernel_init};
use crate::include::kernel::die;
use drivers::{x86_64};

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        die();
    }

    x86_64::init();
    cmdline::cmdline_init();
    init::init();
    kernel_init();
    die()
}
