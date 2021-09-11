fn main() {
    println!("cargo:rerun-if-changed=src/main.c");
    cc::Build::new()
        .file("src/main.c")
        .compile("grub_main");
}
