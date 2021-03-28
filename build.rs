use std::env::{set_current_dir, var};
use std::path::{Path};
use std::process::{Command};
use std::fs::rename;

fn x86_64_bootloader() {
    let header_file = "arch/x86_64/src/boot/header.S";
    println!("cargo:rerun-if-changed={}", header_file);
    Command::new("as")
        .arg("-o")
        .arg("arch/x86_64/src/boot/header.o")
        .arg(header_file)
        .spawn();
}

fn x86_64_bios() {
    let print_file = "arch/x86_64/src/boot/bios/print.c";
    println!("cargo:rerun-if-changed={}", print_file);
    Command::new("gcc")
        .arg("-c")
        .arg(print_file)
        .arg("-o")
        .arg("arch/x86_64/src/boot/bios/print.S")
        .spawn();
}

fn x86_64() {
    x86_64_bootloader();
    x86_64_bios();
}

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    #[cfg(any(target_arch = "x86_64"))]
    x86_64();
}