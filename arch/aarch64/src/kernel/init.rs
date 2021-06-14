#[no_mangle]
pub unsafe extern "C" fn aarch64_kernel_init() -> ! {
    loop { asm!("wfe"); }
}
