use crate::include::asm::hlt;

extern "C" {
    pub fn boot_method() -> &'static str;
}

pub static mut BOOT: &'static str = "";

pub unsafe fn die() -> ! {
    loop { hlt(); }
}

pub unsafe fn boot_init() {
    BOOT = boot_method();
}
