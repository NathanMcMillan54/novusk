extern crate build;
use build::{*};

fn main() {
    as_object("src/boot/multiboot.S", "src/boot/multiboot.o");
    as_object("src/boot/header.S", "src/boot/header.o");
}
