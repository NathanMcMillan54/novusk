#![no_main]
#![no_std]

use core::panic::PanicInfo;

extern crate xtensa_novusk;

use esp32_fw::EspLed;

#[no_mangle]
pub extern "C" fn app_main() -> ! {
    let mut led = EspLed::new();
    led.set_high().unwarp();
    loop {  }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}
