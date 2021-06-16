global_asm!(include_str!("init.S"));

#[no_mangle]
pub extern "C" fn setup() -> ! {
    loop {  }
}
