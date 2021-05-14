/* Everything in kernel needs to be called externally because cargo will think you're calling from
 * the kernel library
 */

pub mod init;
pub mod modules;
pub mod printk;
pub mod userspace;

pub static mut KERNEL_INFO: bool = true;
