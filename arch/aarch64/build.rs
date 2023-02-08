use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use difi::add_dif;

fn main() {
    add_dif("src/include/dif/rpi3b.dif");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    fs::copy("build/libentry.a", out_dir.join("libentry.a"));

    // println!("cargo:rustc-link-lib=static=libentry.a");
    // println!("cargo:rustc-link-search={}", out_dir.display());
}
