extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/kernel/mmio.c");
    cc::Build::new()
        .file("src/kernel/mmio.c")
        .compile("mmio");
}