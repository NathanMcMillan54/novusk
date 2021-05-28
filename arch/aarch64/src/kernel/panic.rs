use core::panic::PanicInfo;

#[cfg(feature = "uefi_rpi3")]
#[cfg(feature = "default")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
