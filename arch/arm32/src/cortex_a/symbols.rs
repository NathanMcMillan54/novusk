// Symbols for Cortex A cpus

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {  }

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {  }

#[no_mangle]
pub extern fn _Unwind_Resume() { loop {  } }
