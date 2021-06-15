#[no_mangle]
pub unsafe extern "C" fn x86_kernel_init() -> ! {
    loop { asm!("hlt"); }
}
