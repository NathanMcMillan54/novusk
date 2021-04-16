use std::process::Command;

pub fn as_object(file: &str, object: &str) {
    println!("Creating object file from {}", file);
    Command::new("as")
        .arg("-o")
        .arg(object)
        .arg(file)
        .spawn();
}

pub fn gcc_object(file: &str, object: &str) {
    println!("Creating object file from {}", file);
    Command::new("gcc")
        .arg("-c")
        .arg(file)
        .arg("-o")
        .arg(object)
        .spawn();
}

pub fn curl(command: &str) {
    println!("Running command with curl");
    Command::new("curl")
        .arg(command)
        .spawn();
}
