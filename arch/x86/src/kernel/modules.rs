use crate::modules::init;

pub unsafe fn x86_modules_init() {
    #[cfg(target_arch = "x86_64")]
    init::x64_start();

    #[cfg(target_arch = "x86")]
    init::x86_start();
}
