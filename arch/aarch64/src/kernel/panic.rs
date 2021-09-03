use crate::aarch64_printk;
use crate::include::asm::wfe;
use core::panic::PanicInfo;
use libbmu::Time;
use rpi::led::RpiLed;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    aarch64_printk!("\nAarch64 kernel panicked");
    aarch64_printk!("    Message: {:?}", _info.message().unwrap());
    aarch64_printk!("    Location: {:?}", _info.location().unwrap());

    let mut led = RpiLed::new();
    led.init();

    loop {
        led.blink(99999999);
        aarch64_printk!("blinked");
    }
}
