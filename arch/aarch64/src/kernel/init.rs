use arm::include::asm;

pub unsafe fn aarch64_init() -> ! {
    aarch64_end_kernel()
}

#[no_mangle]
pub extern "C" fn aarch64_end_kernel() -> ! {
    unsafe { asm::wfe() }
}
