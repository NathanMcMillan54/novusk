use super::modules::{module_end, module_start};

pub unsafe fn module_init(start: (), module_name: &'static str, module_author: &'static str) {
    module_start(module_name, module_author);
    start;
}

pub unsafe fn module_exit(end: ()) {
    end;
    module_end();
}
