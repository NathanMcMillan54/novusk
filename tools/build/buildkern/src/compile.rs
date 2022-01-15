use std::ops::Add;
use std::process::Command;
use crate::fields::*;

pub(crate) fn compile_field(field: String) {
    let mut valid_field = false;
    let mut field_name = field.replace("=", "");
    let mut field_path = String::new();

    for f in 0..NUM_OF_FIELDS {
        if field_name.starts_with(FIELDS[f]) {
            valid_field = true;
            field_path = field_name.replace(FIELDS[f], "");
        }
    }

    if valid_field == false {
        panic!("{} field name is invalid", field_name);
    } else {
        println!("Compiling {}...", field_path);
        Command::new("make")
            .arg("-C")
            .arg(field_path.as_str())
            .arg("all")
            .spawn();
    }
}
