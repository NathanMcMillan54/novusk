fn main() {
    println!("cargo:rerun-if-changes=arch/x86_64/src/boot/grub.ld");
    //println!("cargo:rustc-link-arg=-Tarch/x86_64/src/boot/grub.ld");
}