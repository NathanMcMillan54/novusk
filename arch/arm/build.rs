use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn stellaris_setup() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("stellais_memory.x"))
        .unwrap();
}

fn st_setup() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("st_memory.x"))
        .unwrap();
}

fn main() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // #[cfg(feature = "stellaris_6965")]
    // Because Qemu doesn't support STM32 (yet), for now use the Stellaris memory map and machine
    // for running it
    stellaris_setup();

    // #[cfg(feature = "stm32f4")]
    // st_setup();

    println!("cargo:rustc-link-search={}", out_dir.display());
}
