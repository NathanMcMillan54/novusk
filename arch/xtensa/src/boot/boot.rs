use crate::kernel::device::device;

#[entry]
fn main() -> ! {
    dprintln!("Starting {} kernel...", device());
    loop {  }
}
