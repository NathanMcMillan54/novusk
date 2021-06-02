use crate::{UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION};

pub unsafe fn print_uefi_init() {
    printk!("   UEFI version: {}.{}", UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION);
    printk!("   UEFI graphics initialized");
    printk!("   UEFI FAT Fs is set");
    printk!("   UEFI stdin initialized");
}