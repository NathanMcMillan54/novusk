use difi::add_dif;

fn main() {
    println!("cargo:rustc-link-flag=+crt-static");
    add_dif("src/include/dif/default-x86_64-dif.dif");

    println!("cargo:rerun-if-changed=src/kernel/utils.S");
    cc::Build::new()
        .file("src/kernel/utils.S")
        .compile("utils.o");
}
