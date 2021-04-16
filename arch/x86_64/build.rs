extern crate build;
use build::{*};

fn main() {
    println!("cargo:rerun-if-changed=boot/boot/header.S");
    as_object("src/boot/header.S", "src/boot/header.o");
}