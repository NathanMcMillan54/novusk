use super::{cmdline, cpu};
use drivers::{text::vga, x86_64};
use crate::include::kernel::{die};
use crate::akernel::init;

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
