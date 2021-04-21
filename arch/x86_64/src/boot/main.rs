use super::{cmdline, cpu};
use crate::akernel::init;
use crate::include::kernel::die;
use drivers::{text::vga, x86_64};

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        die();
    }

    vga::init();
    cmdline::cmdline_init();
    x86_64::init();
    init::init();

    die()
}
