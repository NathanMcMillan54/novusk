pub mod id;
pub use id::BRAND;

use kinfo::status::set_status;

pub unsafe fn cpu_init() {
    gdt::gdt_init();

    if BRAND == "AMD" {
        amd::amd_init();
    } else if BRAND == "Intel" {
        intel::intel_init();
    } else {
        set_status("not ok");
        early_printk!("No CPU brand to setup");
    }
}

pub mod amd;
pub mod gdt;
pub mod intel;
