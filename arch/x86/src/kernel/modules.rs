use novusk::module::{module_end, module_init};

#[cfg(target_arch = "x86")]
pub unsafe fn x86_modules_init() {

}

#[cfg(target_arch = "x86_64")]
pub unsafe fn x64_modules_init() {
    use ex1::{ex1_exit, ex1_init};

    module_init(ex1_init(), "Nathan McMillan <nathanmcmillan54@gmail.com>", "ex1");
    module_end(ex1_exit());
}

pub unsafe fn modules_init() {
    #[cfg(target_arch = "x86")]
    x86_modules_init();

    #[cfg(target_arch = "x86_64")]
    x64_modules_init();
}
