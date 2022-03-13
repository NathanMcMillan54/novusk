use difi::add_kernel_dif;
use std::fs::remove_file;
use std::path::Path;

fn main() {
    if Path::new("src/dif.rs").exists() {
        remove_file("src/dif.rs");
    }

    // Only support Aarch64 for now
    add_kernel_dif("../../../arch/aarch64/src/include/dif/kernel_dif.dif");
}
