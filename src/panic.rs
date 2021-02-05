use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    kprint!("{}\n", _info);
    loop {  }
}
