use crate::drivers::uart::uart0::Uart0;
use core::str::from_utf8_unchecked;
use super::st::st;

#[cfg(feature = "uefi_kernel")]
pub use sys::*;

fn bm_write(bytes: &[u8]) {
    let mut writer = Uart0;
    writer.write_bytes(bytes);
}

#[no_mangle]
pub unsafe extern "C" fn write(bytes: &[u8]) {
    #[cfg(not(feature = "uefi_kernel"))]
    bm_write(bytes);

    #[cfg(feature = "uefi_kernel")]
    printk!("{}", from_utf8_unchecked(bytes));
}
