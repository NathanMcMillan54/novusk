#![no_std]
#![no_main]

pub use novusk::info;

#[cfg(target_arch = "x86_64")]
pub use novusk::x86;

#[cfg(target_arch = "x86")]
pub use novusk::x86;

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    loop {  }
}
