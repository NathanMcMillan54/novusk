use std::env;
use std::fs::{copy, File};
use std::io::Write;
use std::path::PathBuf;

pub fn add_mem_file(file: &[u8]) {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(file)
        .unwrap();


    println!("cargo:rustc-link-search={}", out.display());
}
