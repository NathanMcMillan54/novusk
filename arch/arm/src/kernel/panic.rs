use core::panic::PanicInfo;

#[cfg(not(feature = "cortex_m"))]
#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {  }
}
