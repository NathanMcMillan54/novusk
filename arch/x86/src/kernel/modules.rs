use modules::*;

pub unsafe fn x86_modules_init() {
    #[cfg(target_arch = "x86_64")]
    x64_start();

    #[cfg(target_arch = "x86")]
    x86_start();
}
