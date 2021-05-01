#[no_mangle]
pub unsafe extern "C" fn halt() -> ! {
    asm!("hlt");
    halt()
}
