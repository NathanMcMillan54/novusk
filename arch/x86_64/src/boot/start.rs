use bootloader::BootInfo;
use core::arch::asm;
use core::borrow::Borrow;
use core::fmt::Write;
use core::ops::Add;
use core::ptr::{write_volatile};
use bootloader::bootinfo::{MemoryRegion, MemoryRegionType};
use raw_cpuid::CpuId;
use novuskinc::kernel::setup_arch;
use x86_64::instructions::port::Port;
use crate::early_printk;
use crate::boot::cpu::{APIC, PIC, X2APIC, X86_64CPU};
use crate::boot::early_vga::{VGA_WRITER, VgaWriter};
use crate::boot::video::*;

unsafe fn check_cpuid() {
    let mut cpuid = CpuId::new();

    #[cfg(feature = "uefi_boot")]
    if cpuid.get_feature_info().unwrap().has_apic() {
        X86_64CPU.interrupt_handler = APIC;
    } else if cpuid.get_feature_info().unwrap().has_x2apic() {
        X86_64CPU.interrupt_handler = X2APIC;
    }

    #[cfg(feature = "bios_boot")]
    {
        X86_64CPU.interrupt_handler = PIC;
    }

    if X86_64CPU.interrupt_handler == 4 {
        panic!("Could not set interrupt_handler");
    }

    match cpuid.get_vendor_info().expect("Could not get CPU vendor info").as_str() {
        "GenuineIntel" => {
            X86_64CPU.brand.replace("GenuineIntel");
        },
        "AuthenticAMD" => {
            X86_64CPU.brand.replace("AuthenticAMD");
        }
        _ => { panic!("Could not get CPU vendor info"); }
    }
}

/// Booting with ``bootloader`` < v0.10.0
#[cfg(feature = "bootloader_rs_0_9_23")]
#[no_mangle]
#[inline]
pub unsafe extern "C" fn _start(bootinfo: &'static BootInfo) -> ! {
    check_cpuid();
    set_video_driver(VGA);

    early_printk!("Starting Novusk...\n");
    early_printk!("{:?}\n", X86_64CPU);
    early_printk!("Using VGA Text buffer\n");

    setup_arch();

    panic!()
}
