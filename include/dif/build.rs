#[path = "../../tools/build/rcc.rs"]
mod rcc;

use rcc::cc_cmd;

fn main() {
    println!("cargo:rerun-if-changed=src/trim.c");
    ccbuild::Gcc::new()
        .file("src/trim.c")
        .output("trim.o")
        .cc_arch(cc_cmd())
        .compile();
}
