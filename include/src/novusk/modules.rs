pub(crate) unsafe fn module_start(name: &'static str, author: &'static str) {
    MODULE = name;
    AUTHOR = author;
}

pub(crate) unsafe fn module_end() {
    MODULE = "";
    AUTHOR = "";
}

pub static mut MODULE: &str = "";
pub static mut AUTHOR: &str = "";
