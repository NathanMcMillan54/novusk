use crate::include::asm::wfe;

pub unsafe fn setup() -> ! {
    wfe();
}
