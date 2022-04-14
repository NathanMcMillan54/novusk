extern "C" {
    pub fn aarch64_boot_setup();
    pub fn aarch64_kernel_setup();
    pub fn aarch64_kernel_init();
    pub fn aarch64_setup_early_printk();
    pub fn aarch64_irq_setup();
}

pub use aarch64_boot_setup as arm64_boot_setup;
pub use aarch64_kernel_setup as arm64_kernel_setup;
pub use aarch64_kernel_init as arm64_kernel_init;
pub use aarch64_setup_early_printk as arm64_setup_early_printk;
pub use aarch64_irq_setup as arm64_irq_setup;
