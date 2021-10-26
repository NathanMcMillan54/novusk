use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn stellaris_init() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("stellaris_memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.x");
}


fn main() {
    // #[cfg(feature = "stellaris_6965")]
    stellaris_init();
}
