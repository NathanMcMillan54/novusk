// Lang items for Rust
#[panic_handler]
fn _panic(_info: &core::panic::PanicInfo) -> ! {
    loop {  }
}

#[lang = "eh_personality"]
fn eh_personality() {   }
