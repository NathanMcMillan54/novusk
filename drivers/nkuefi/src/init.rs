#[cfg(target_arch = "x86_64")]
unsafe fn x86_64_init() {
    use crate::x86::init;
    init();
}

pub unsafe fn uefi_init() {
    #[cfg(target_arch = "x86_64")]
    x86_64_init();
}
