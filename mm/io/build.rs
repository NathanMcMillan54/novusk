extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/mmio.c");
    cc::Build::new()
        .file("src/mmio.c")
        .compile("mmio");
}
