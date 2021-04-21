extern crate cc;

pub fn gas(file: &str, object: &str) {
    cc::Build::new()
        .file(file)
        .compile(object);
}

pub fn gc(file: &str, object: &str) {
    cc::Build::new()
        .file(file)
        .compile(object);
}
