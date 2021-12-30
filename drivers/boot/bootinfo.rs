/// ``BootInfo`` is a struct that contains information created by a bootloader that the kernel will
/// use, this should be passed as an argument to the "main" function of the kernel or stored in an
/// address that can be read from the kernel.
pub struct BootInfo {
    pub bootloader_name: &'static str,
    pub boot_method: &'static str,
    pub boot_mem: [usize; 64],
}
