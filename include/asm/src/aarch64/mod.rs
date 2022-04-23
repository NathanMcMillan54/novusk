pub mod ints;

pub mod io {
    use core::arch::global_asm;

    global_asm!(include_str!("io.S"));

    extern "C" {
        pub fn inb(port: u32) -> u32;
        pub fn outb(port: u32, out: u16);
    }
}
