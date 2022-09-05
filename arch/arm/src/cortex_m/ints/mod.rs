pub fn enable_interrupts() {
    unsafe { cortex_m::interrupt::enable(); }
}