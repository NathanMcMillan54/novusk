#![allow(warnings)]

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub mod compile;
pub mod fields;

fn main() {
    let args: Vec<String> = env::args().collect();

    let konfig_string_path = args[1].clone();

    if !Path::new(konfig_string_path.as_str()).exists() {
        panic!("{} is an invalid kernel config path", konfig_string_path);
    }

    let konfig = File::open(konfig_string_path).unwrap();
    let konfig_reader = BufReader::new(konfig.try_clone().unwrap());

    for config_field in konfig_reader.lines() {
        if !config_field.as_ref().unwrap().starts_with("#") {
            compile::compile_field(config_field.unwrap());
        }
    }
}
