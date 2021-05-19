use super::blink::blink_init;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    blink_init();
    dprintln!("Blink initialized");

    // TODO: Loop with Xtensa assembly
    loop {  }
}
