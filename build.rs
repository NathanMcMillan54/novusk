use std::env::{set_current_dir, var};
use std::path::{Path};
use std::process::{Command};
use std::fs::rename;

fn x86_64_boot() {
    let header_file = "arch/x86_64/src/boot/header.S";
    let bioscall_file = "arch/x86_64/src/boot/bioscall.S";
    println!("cargo:rerun-if-changed={}", header_file);
    Command::new("as")
        .arg("-o")
        .arg("arch/x86_64/src/boot/header.o")
        .arg(header_file)
        .spawn();
    println!("cargo:rerun-if-changed={}", bioscall_file);
    Command::new("as")
        .arg("-o")
        .arg("arch/x86_64/src/boot/bioscall.o")
        .arg(bioscall_file)
        .spawn();
}

fn x86_64() {
    x86_64_boot();
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    #[cfg(any(target_arch = "x86_64"))]
    x86_64();
}