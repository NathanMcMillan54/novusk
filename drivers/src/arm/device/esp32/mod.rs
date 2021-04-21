pub unsafe fn esp32_init() {
    loop {
        asm!("wfe");
    }
}
