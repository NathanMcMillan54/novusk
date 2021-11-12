use crate::aarch64_printk;
use crate::include::asm::wfe;
use crate::mm::map::print_memory_map;
use core::panic::PanicInfo;
use libbmu::Time;
use rpi::rpi3::led::RpiLed;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    aarch64_printk!("\nAarch64 kernel panicked\n");
    aarch64_printk!("    Message: {:?}\n", _info.message().unwrap());
    aarch64_printk!("    Location: {:?}\n", _info.location().unwrap());

    print_memory_map();

    let mut led = RpiLed::new();
    led.init();

    loop {
        led.blink(500000);
    }
}
