use arm::include::asm;

#[no_mangle]
pub extern "C" fn aarch64_end_kernel() -> ! {
    unsafe { asm::wfe() }
}
