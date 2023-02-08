fn aarch64_build() {
    println!("cargo:rustc-link-lib=static=libentry.a");
}

fn main() {
    #[cfg(target_arch = "aarch64")]
    aarch64_build();
}
