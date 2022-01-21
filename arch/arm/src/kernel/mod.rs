pub mod arm_init;
pub mod cpu;
pub mod panic;
pub mod setup;

pub use arm_init::arm_kernel_init;
pub use setup::setup_arm_kernel;

/*pub mod arm_init;
pub mod board;
pub mod io;
pub mod irq;
pub mod printk;
pub mod setup;

#[path = "../../../../kernel/device.rs"]
pub mod device;

#[path = "../../../../kernel/irq.rs"]
pub mod irqs;
*/