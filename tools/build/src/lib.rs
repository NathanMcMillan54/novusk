use std::process::Command;

pub fn as_object(file: &str, object: &str) {
    Command::new("as")
        .arg("-o")
        .arg(object)
        .arg(file)
        .spawn();
}

pub fn gcc_object(file: &str, object: &str) {
    Command::new("gcc")
        .arg("-c")
        .arg(file)
        .arg("-o")
        .arg(object)
        .spawn();
}

// This doesn't work for some reason
pub fn curl(command: &str) {
    Command::new("curl")
        .arg(command)
        .spawn();
}

pub fn nasm_object(file: &str) {
    Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(file)
        .spawn();
}