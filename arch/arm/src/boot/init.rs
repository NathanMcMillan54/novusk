use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    unsafe {
        loop { asm!("wfe"); }
    }
}
