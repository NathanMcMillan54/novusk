#[no_mangle]
pub unsafe fn die() -> ! {
    loop { asm!("hlt"); }
}
