use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn arm_ld_setup() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("src/boot/start/stellaris6965_mem.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=src/boot/start/stellaris_mem.x");
}

fn main() {
    println!("cargo:rerun-if-changed=src/boot/start/linker.ld");
    println!("cargo:rerun-if-changed=src/boot/start/a53_start.S");
    println!("cargo:rerun-if-changed=src/boot/start/a7_start.S");
    println!("cargo:rerun-if-changed=src/boot/start/cm_boot32.S");

    arm_ld_setup();
}
