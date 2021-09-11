fn main() {
    println!("cargo:rerun-if-changed=src/stdio.c");
    cc::Build::new()
        .file("src/stdio.c")
        .compile("stdio");
}