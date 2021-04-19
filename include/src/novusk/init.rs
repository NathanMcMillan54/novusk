use super::module::{end_module, module_setup, RUNNING_MODULE, RUNNING_MODULE_AUTHOR};

pub unsafe fn module_init(start_function: (), module_name: &'static str, module_author: &'static str) {
    module_setup(module_name, module_author);
    start_function;
}

pub unsafe fn module_exit(end_function: ()) {
    end_function;
    end_module();
}
