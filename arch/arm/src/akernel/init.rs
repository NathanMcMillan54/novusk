pub unsafe fn init() -> ! {
    loop { asm!("wfe"); }
}
