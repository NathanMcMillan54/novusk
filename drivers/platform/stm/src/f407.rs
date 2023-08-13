pub mod gpio;
pub mod uart;

#[no_mangle]
pub extern "C" fn early_device_init() {
    gpio::gpio_init();
    uart::uart_init();
}
