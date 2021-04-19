pub static mut RUNNING_MODULE: &str = "";
pub static mut RUNNING_MODULE_AUTHOR: &str = "";

pub(crate) unsafe fn module_setup(module: &'static str, author: &'static str) {
    RUNNING_MODULE = module;
    RUNNING_MODULE_AUTHOR = author;
}

pub(crate) unsafe fn end_module() {
    RUNNING_MODULE = "";
    RUNNING_MODULE_AUTHOR = "";
}
