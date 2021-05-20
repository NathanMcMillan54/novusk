pub mod blink;
use net::net_init;

pub unsafe fn drivers_init() {
    net_init();
    blink::blink();
    dprintln!("Initialized network drivers");
}
