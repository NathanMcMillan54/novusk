#![no_std]
#![no_main]


#[cfg(target_arch = "x86_64")]
pub use novusk::x86;

#[cfg(target_arch = "x86")]
pub use novusk::x86;

#[cfg(target_arch = "aarch64")]
pub use novusk::aarch64;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {

}
