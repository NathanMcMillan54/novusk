use novuskinc::irq::IrqChip;
use raw_cpuid::CpuId;
use crate::early_printk;

extern "C" {
    static mut IRQCHIP: IrqChip;
}

pub fn validate_cpu() -> bool {
    let cpuid = CpuId::new();

    let brand = cpuid.get_vendor_info();
    let feature = cpuid.get_feature_info();

    if brand.is_none() {
        early_printk!("CPU brand can't be detected\n");
        return false;
    }

    match brand.as_ref().unwrap().as_str() {
        "GenuineIntel" => {  },
        "AuthenticAMD" => {  },
        _ => {
            early_printk!("Unknown CPU brand, {}\n", brand.unwrap().as_str());
            return false;
        }
    }

    unsafe {
        if IRQCHIP.name.contains("APIC") {
            if !feature.unwrap().has_apic() {
                panic!("This device do not have APIC capabilities, the IRQ chip driver was compiled for APIC");
            }
        }
    }

    return true;
}
