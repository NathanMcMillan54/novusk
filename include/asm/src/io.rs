extern "C" {
    /// Writes ``out`` value to ``port``.
    pub fn outb(port: u32, out: u16);
    /// Reads from ``port``.
    pub fn inb(port: u32) -> u32;
}
