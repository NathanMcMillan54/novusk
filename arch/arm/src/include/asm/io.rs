#[no_mangle]
pub extern "C" fn outb(port: u32, out: u16) {  }

#[no_mangle]
pub extern "C" fn inb(port: u32) -> u32 { 0x0 }
