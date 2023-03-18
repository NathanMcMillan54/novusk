use super::setup::boot_setup;
use raw_cpuid::CpuId;
use crate::kernel::kernel::{gop_init, vga_init};

fn validate_cpu() {
    let cpuid = CpuId::new();
}

fn setup_bootloader() {
    #[cfg(feature = "bios_boot")]
    bios_bootloader_setup();

    #[cfg(feature = "uefi_boot")]
    uefi_bootloader_setup();
}

#[cfg(feature = "bios_boot")]
fn bios_bootloader_setup() {
    vga_init();
}

#[cfg(feature = "uefi_boot")]
fn uefi_bootloader_setup() {
    gop_init();
    nkuefi::nkuefi_setup();
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    //validate_cpu();
    setup_bootloader();
    boot_setup();

    loop { core::arch::asm!("nop") }
}
