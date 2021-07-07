use cortex_m_rt::entry;

#[entry]
pub unsafe extern "C" fn main() -> ! {
    loop { asm!("wfe"); }
}
