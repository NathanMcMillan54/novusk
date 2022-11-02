use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use difi::add_dif;

#[cfg(feature = "rpi2")]
const DIF: &'static str = "src/include/dif/rpi2.dif";

#[cfg(feature = "stellaris_6965")]
const DIF: &'static str = "src/include/dif/stellaris6965.dif";

fn cortex_m_build_setup() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("stellaris_mem.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}

fn main() {
    // Reruns
    println!("cargo:rerun-if-changed=stellaris_mem.x");
    println!("cargo:rerun-if-changed=src/boot/start/linker.ld");
    println!("cargo:rerun-if-changed=src/boot/start/a7_start.S");

    println!("cargo:rerun-if-changed={}", DIF);
    add_dif(DIF);

    #[cfg(feature = "cortex_m")]
    cortex_m_build_setup();
}
