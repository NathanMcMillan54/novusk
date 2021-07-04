#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    loop { asm!("wfe"); }
}
