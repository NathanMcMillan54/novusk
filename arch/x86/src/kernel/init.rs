#[no_mangle]
pub unsafe extern "C" fn x86_kernel_init() -> ! {
    efi_write!("Hello world!");
    loop { asm!("hlt"); }
}
