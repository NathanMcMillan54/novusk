use crate::include::asm::wfe;

#[entry]
fn init() -> ! {
    unsafe {
        wfe();
    }
}