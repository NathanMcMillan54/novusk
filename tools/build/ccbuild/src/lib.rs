use std::env;
use std::fs::{copy, File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub struct Gcc {
    pub cc_arch: Option<&'static str>,
    pub input_file: Option<&'static str>,
    pub output_file: Option<&'static str>,
    pub args: Option<[&'static str; 5]>,
}

impl Gcc {
    pub fn new() -> Self {
        return Gcc {
            cc_arch: None,
            input_file: None,
            output_file: None,
            args: None,
        }
    }

    pub fn cc_arch(&mut self, arch: &'static str) -> &mut Self {
        self.cc_arch = Some(arch);
        return self;
    }

    pub fn file(&mut self, file_path: &'static str) -> &mut Self {
        self.input_file = Some(file_path);

        return self;
    }

    pub fn output(&mut self, file_name: &'static str) -> &mut Self {
        self.output_file = Some(file_name);

        return self;
    }

    pub fn arguments(&mut self, args: [&'static str; 5]) -> &mut Self {
        self.args = Some(args);

        return self;
    }

    pub fn compile(&mut self) {
        if self.cc_arch.is_none() {
            panic!("Cross Compile Architecture is not set");
        } else if self.input_file.is_none() {
            panic!("Input File not set");
        } else if self.output_file.is_none() {
            self.output_file = Some("output");
            println!("cargo:warning={}", "Output file is not set, defaulting to \"output\"");
        } else if self.args.is_none() {
            self.args = Some(["", "", "", "", ""]);
        }

        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

        Command::new(format!("{}{}", self.cc_arch.unwrap(), "-gcc").as_str())
            .arg(self.input_file.unwrap())
            .arg("-c")
            .args(self.args.unwrap())
            .args(["-o", format!("{:?}{}", out_dir, self.output_file.unwrap()).as_str()])
            .spawn();

        copy(self.output_file.unwrap(), out_dir.join(self.output_file.unwrap()));
    }
}
