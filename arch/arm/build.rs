use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;

#[cfg(feature = "lm3s6965")]
const MEM_PATH: &str = "src/include/dev/lm3s6965_memory.x";

#[cfg(feature = "stm32f407")]
const MEM_PATH: &str = "src/include/dev/stm32f407_memory.x";

#[cfg(feature = "cortex_m_device")]
fn cortex_m_kernel_setup() {
    let mem_file_contents = read_to_string(MEM_PATH).unwrap();
    let file_bytes = mem_file_contents.as_bytes();

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(file_bytes)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed={}", MEM_PATH);
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
}

fn main() {
    #[cfg(feature = "cortex_m_device")]
    cortex_m_kernel_setup();
}
