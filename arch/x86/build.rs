use std::env;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::env::set_current_dir;

fn compile_grub_boot() {
    println!("cargo:rerun-if-changed=src/boot/grub.S");
    Command::new("as")
        .args(&["-32", "src/boot/grub.S", "-o", PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("grub.o").to_str().unwrap()])
        .spawn()
        .unwrap();
}

fn main() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).to_str();

    #[cfg(feature = "grub")]
    compile_grub_boot();
}
