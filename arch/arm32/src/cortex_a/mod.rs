pub mod ints;

pub fn heap_start() -> usize {
    return 0;
}

pub fn cortex_a_init() -> (&'static str, u32) {
    return ("Cortex-AXX", 0);
}
