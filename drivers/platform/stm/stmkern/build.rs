#[path = "../../../../arch/arm/src/tools/build_mem.rs"]
mod build_mem;

fn main() {
    println!("cargo:rerun-if-change=stm32407_mem.x");
    build_mem::add_mem_file(include_bytes!("stm32f407_mem.x"));
}
