pub mod io {
    core::arch::global_asm!(include_str!("io.S"));

    extern "C" {
        pub fn outb(port: u32, out: u16);
        pub fn inb(port: u32) -> u32;
    }
};

pub mod irq;