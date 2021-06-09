use core::str::from_utf8_unchecked;
use super::st::st;

#[cfg(feature = "uefi_kernel")]
pub use sys::*;

#[no_mangle]
pub unsafe extern "C" fn write(bytes: &[u8]) {
    #[cfg(feature = "uefi_kernel")]
    printk!("{}", from_utf8_unchecked(bytes));
}
