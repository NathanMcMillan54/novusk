use difi::add_dif;

fn main() {
    //println!("cargo:rustc-link-arg=-fPIE");
    add_dif("src/include/dif/default-x86_64-dif.dif");
}
