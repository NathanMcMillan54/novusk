use crate::aarch64_printk;
use crate::include::asm::wfe;
use crate::mm::map::print_memory_map;
use core::panic::PanicInfo;
use libbmu::Time;
use rpi::led::Rpi3Led;

#[panic_handler]
pub unsafe fn panic(info: &PanicInfo) -> ! {
    aarch64_printk!("\nAarch64 kernel panicked\n");
    aarch64_printk!("    Message: {:?}\n", info.message().unwrap());
    aarch64_printk!("    Location: {:?}\n", info.location().unwrap());

    print_memory_map();

    let mut led = Rpi3Led::new();
    led.init();

    loop {
        led.blink(500000);
    }
}
