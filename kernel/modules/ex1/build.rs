fn main() {
    println!("cargo:rerun-if-changed=src/add.c");
    cc::Build::new()
        .file("src/add.c")
        .compile("add");
}