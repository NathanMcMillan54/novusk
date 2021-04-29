extern crate build;
use build::gas_object;

fn main() {
    println!("cargo:rerun-if-changed=src/boot/header.S");
    gas_object("src/boot/header.S");
}
