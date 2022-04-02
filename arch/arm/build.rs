use std::env;
use std::fs::{copy, File};
use std::io::Write;
use std::path::PathBuf;

fn arm_ld_setup() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(mem::MEM_FILE)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", mem::MEM_FILE_PATH);

    copy(mem::MEM_FILE_PATH, "src/boot/start/memory.x");
}

fn main() {
    println!("cargo:rerun-if-changed=src/boot/start/linker.ld");
    println!("cargo:rerun-if-changed=src/boot/start/a53_start.S");
    println!("cargo:rerun-if-changed=src/boot/start/a7_start.S");
    println!("cargo:rerun-if-changed=src/boot/start/cm_boot32.S");

    arm_ld_setup();
}

mod mem {
    #[cfg(feature = "stellaris6965")]
    pub const MEM_FILE: &[u8] = include_bytes!("../../drivers/platform/stellaris/stellaris6965_mem.x");

    #[cfg(feature = "stellaris6965")]
    pub const MEM_FILE_PATH: &'static str = "../../drivers/platform/stellaris/stellaris6965_mem.x";

    #[cfg(feature = "stm32f407")]
    pub const MEM_FILE: &[u8] = include_bytes!("../../drivers/platform/stm/stm32f407_mem.x");

    #[cfg(feature = "stm32f407")]
    pub const MEM_FILE_PATH: &'static str = "../../drivers/platform/stm/stm32f407_mem.x";
}
