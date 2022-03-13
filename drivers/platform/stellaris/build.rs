use difi::add_kernel_dif;
use std::fs::remove_file;
use std::path::Path;

fn main() {
    if Path::new("src/dif.rs").exists() {
        remove_file("src/dif.rs");
    }

    add_kernel_dif("../../../arch/arm/src/include/dif/kernel_dif.dif");
}
