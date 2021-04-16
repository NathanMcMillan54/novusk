use super::kernel_init;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    kernel_init();
    loop { asm!("hlt"); }
}
