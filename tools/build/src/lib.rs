use std::process::Command;

pub fn gas_object(file: &str) {
    let output = file.replace(".S", ".o");
    Command::new("as")
        .arg("-o")
        .arg(output)
        .arg(file)
        .spawn();
}
