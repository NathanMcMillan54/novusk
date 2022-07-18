use std::borrow::BorrowMut;
use std::ops::Add;
use std::process::Command;
use crate::fields::*;

pub(crate) fn compile_field(field: String) {
    let mut valid_field = false;
    let mut field_name = field;
    let mut config_arguments = String::new();

    for f in 0..NUM_OF_FIELDS {
        if field_name.starts_with(FIELDS[f]) {
            valid_field = true;
            let mut valid_field_name = FIELDS[f].to_string();
            valid_field_name.push_str("=");
            config_arguments = field_name.replace(valid_field_name.as_str(), "");
        }
    }

    let mut arguments: Vec<&str> = config_arguments.split(" ").collect();

    if valid_field == false {
        panic!("{} field name is invalid", field_name);
    } else {
        println!("Compiling {}...", arguments[0]);
        let mut command = Command::new("cargo");

        command.args(&["rustc", "-p"]);

        for a in 0..arguments.len() {
            command.arg(arguments[a]);
        }

        command.spawn();
    }
}
