pub mod id;
pub use id::BRAND;

use kinfo::info::set_info;

pub unsafe fn cpu_init() {
    if BRAND == "AMD" {
        amd::amd_init();
    } else if BRAND == "Intel" {
        intel::intel_init();
    } else {
        set_info("not ok");
        crate::x86_printk!("No CPU brand to setup");
    }
}

pub mod amd;
pub mod intel;
