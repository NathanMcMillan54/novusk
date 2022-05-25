use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

const DIF_STATIC: &'static str = "#[no_mangle]\npub static DIF_FILE: &'static [(&'static str, &'static str); 11] = &";

pub fn add_kernel_dif(dif: &str) {
    let kernel_dif = File::open(dif);
    let rust_dif = File::create("src/dif.rs"); // Should be included in lib.rs for device driver

    if kernel_dif.is_err() {
        panic!("Failed to open kernel DIF file (kernel_dif.dif)");
    } else if rust_dif.is_err() {
        panic!("Failed to create Rust DIF file (src/dif.rs)");
    }

    rust_dif.unwrap().write_fmt(format_args!("{}{}{}", DIF_STATIC, read_to_string(Path::new(dif)).unwrap(), ";\n"));
}
