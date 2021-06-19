use super::x86_init::x86_init;

#[no_mangle]
pub unsafe extern "C" fn x86_main() -> ! {
    x86_init();
    loop {  }
}
