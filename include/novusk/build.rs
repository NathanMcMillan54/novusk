use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn check_file(name: &str) -> bool {
    if Path::new(name).exists() {
        return true;
    } else { return false; }
}

fn main() {
    let kernel_modules = vec!["device", "display"];
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for km in 0..kernel_modules.len() {
        if check_file(format!("{}{}{}", "lib", kernel_modules[km], ".a").as_str()) {
            fs::copy(format!("{}{}{}", "lib", kernel_modules[km], ".a"), out_dir.join(format!("{}{}{}", "lib", kernel_modules[km], ".a")));
            println!("cargo:rerun-if-changed=lib{}.a", kernel_modules[km]);
        } else { panic!("Test"); }
    }

    println!("cargo:rustc-link-search=staticlib=libdevice.a");
}

