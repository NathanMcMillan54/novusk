pub mod id;

use super::kernel::*;
use crate::drivers::amd::amd_init;
use crate::drivers::ix86::intel::intel_init;

pub unsafe fn cpu_init() {
    if id::BRAND == "AMD" {
        amd_init();
    } else if id::BRAND == "Intel" {
        intel_init();
    } else {
        x86_printk!("Unknown CPU, cannot initialize it");
    }
}
