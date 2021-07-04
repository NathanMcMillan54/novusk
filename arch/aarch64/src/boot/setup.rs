use r0::zero_bss;

extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

unsafe fn get_board() -> u8 {
    // TODO: get board info
    asm!("");
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    get_board();
    loop { asm!("wfe"); }
}

#[no_mangle]
pub unsafe extern "C" fn clear_bss() {
    zero_bss(&mut __bss_start, &mut __bss_end);
}
