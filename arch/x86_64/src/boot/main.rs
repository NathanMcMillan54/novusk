use super::{kernel_init};
use crate::kernel::{time::time_init};

unsafe fn protected_mode() {
    asm!(
        "mov ax, 0x13",
        "int 0x10"
    );
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    protected_mode();
    time_init();
    kernel_init()
}
