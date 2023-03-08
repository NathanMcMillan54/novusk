use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use difi::add_dif;

fn main() {
    add_dif("src/include/dif/rpi3b.dif");
}
