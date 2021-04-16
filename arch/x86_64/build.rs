extern crate build;
use build::{*};

fn main() {
    gcc_object("src/boot/header.S", "src/boot/header.o");
    gcc_object("src/boot/kentry32.S", "src/boot/kentry32.o");
}
