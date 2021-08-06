mod grub {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::Command;

    fn compile_grub_boot() {
        println!("cargo:rerun-if-changed=src/boot/grub.S");
        Command::new("as")
            .args(&["-32", "src/boot/grub.S", "-o", PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("grub.o").to_str().unwrap()])
            .spawn()
            .unwrap();
    }

    pub fn build() {
        compile_grub_boot();
    }
}

fn main() {
    #[cfg(feature = "grub")]
    grub::build();
}
