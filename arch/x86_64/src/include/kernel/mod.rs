use super::asm::hlt;

#[no_mangle]
pub unsafe fn die() -> ! {
    loop { hlt(); }
}
