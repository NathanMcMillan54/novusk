use cortex_m_rt::entry;

#[entry]
fn boot() -> ! {
    panic!("STM kernel ended");
}
