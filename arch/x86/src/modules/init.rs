#[cfg(target_arch = "x86_64")]
pub unsafe fn x64_start() {
    use super::{ex1};

    ex1::ex1_init();
    kinfo!("ex1 initialized");
}

#[cfg(target_arch = "x86")]
pub unsafe fn x86_start() {

}
