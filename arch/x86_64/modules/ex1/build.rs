extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/add.S");
    cc::Build::new()
        .file("src/add.S")
        .compile("add");
}