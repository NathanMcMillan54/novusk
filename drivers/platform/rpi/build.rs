use difi::add_kernel_dif;
use std::fs::remove_file;
use std::path::Path;

fn main() {
    // Only support Aarch64 RPi3b for now
    add_kernel_dif("../../../arch/aarch64/src/include/dif/rpi3b.dif");
}
