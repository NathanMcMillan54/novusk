extern crate build;
use build::{*};

fn main() {
    nasm_object("src/boot/header.asm");
    nasm_object("src/boot/multiboot.asm");
    nasm_object("src/boot/kernelEntry.asm");
}
