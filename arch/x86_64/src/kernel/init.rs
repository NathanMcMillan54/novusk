use crate::boot::{kernel_init, kernel_main};

pub unsafe fn init() -> ! {
    kernel_init();
    kernel_main()
}
