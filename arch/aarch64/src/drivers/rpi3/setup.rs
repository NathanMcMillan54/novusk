#[no_mangle]
pub unsafe extern "C" fn rpi_init() -> ! {
    loop { asm!("wfe"); }
}