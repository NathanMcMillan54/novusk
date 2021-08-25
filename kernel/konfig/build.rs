fn main() {
    println!("cargo:rerun-if-changed=src/defconfig.txt");
}