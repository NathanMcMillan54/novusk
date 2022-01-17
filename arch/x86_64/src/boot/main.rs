#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {

    loop { asm!("hlt"); }
}
