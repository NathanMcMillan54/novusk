/*#[cfg(not(target_arch = "aarch64"))]
use unistd::table::*;*/

#[cfg(target_arch = "x86_64")]
fn x86_64() {
    //assert!(READ == 0, "unistd::READ's value is not 0");
    //assert!(WRITE == 1, "unistd::WRITE's value is not 1");
}

#[cfg(target_arch = "aarch64")]
fn aarch64() {
    // assert!(unistd::WRITE == 4, "unistd::WRITE's value is not 4");
}

#[cfg(all(target_arch = "riscv32", target_arch = "riscv64"))]
fn riscv() {
    //assert!(READ == 63, "unistd::READ's value is not 63");
    //assert!(WRITE == 64, "unistd::WRITE's value is not 64");
}

pub(crate) fn check_sys_nums() {
    #[cfg(target_arch = "x86_64")]
    x86_64();

    /*#[cfg(target_arch = "aarch64")]
    aarch64();*/

    #[cfg(all(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv();
}
