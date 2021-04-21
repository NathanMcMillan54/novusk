extern crate build;
use build::gas;

fn main() {
    println!("cargo:rerun-if-changed=src/boot/header.S");
    gas("src/boot/header.S", "header");
}
