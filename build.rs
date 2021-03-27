use std::env::{set_current_dir, var};
use std::path::{Path};
use std::process::{Command};
use std::fs::rename;

fn x86_64_bootloader() {

}

fn x86_64_link() {

}

fn x86_64() {
    println!("cargo:rerun-if-changed=arch/x86_64/src/boot/header.S");
    Command::new("as")
        .arg("-o")
        .arg("arch/x86_64/src/boot/header.o")
        .arg("arch/x86_64/src/boot/header.S")
        .spawn();
    x86_64_link();
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    #[cfg(any(target_arch = "x86_64"))]
    x86_64();
}