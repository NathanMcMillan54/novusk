extern "C" {
    fn mmio_write(reg: u64, val: u32);
    fn mmio_read(reg: i64) -> u64;
}
