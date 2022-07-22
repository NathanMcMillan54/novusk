pub mod cpu;
pub mod early_printk;
pub mod handlers;
pub mod idt;
pub mod io;
pub mod irq;
// pub mod kernel;
pub mod power;
pub mod printk;
pub mod setup;
pub mod time;
pub mod x86_init;

#[path = "../../../../drivers/drivers.rs"]
pub mod kernel_drivers;

#[path = "video/video_vga.rs"]
pub mod video_vga;

