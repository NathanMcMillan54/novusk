use raw_cpuid::CpuId;
use crate::early_printk;

pub fn validate_cpu() -> bool {
    let cpuid = CpuId::new();

    let brand = cpuid.get_vendor_info();

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

    return true;
}
