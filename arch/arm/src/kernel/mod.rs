pub mod arm_init;
pub mod cpu;
pub mod early_printk;
pub mod irq;
pub mod panic;
pub mod serial;
pub mod setup;
pub mod uart;

pub use arm_init::arm_kernel_init;
pub use self::setup::setup_arm_kernel;

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