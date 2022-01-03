// Symbols for Cortex A cpus

// Library symbols
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {  }

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {  }

#[no_mangle]
pub extern "C" fn __sync_val_compare_and_swap_1() {  }

#[no_mangle]
pub extern "C" fn __sync_lock_test_and_set_1() {  }

#[no_mangle]
pub extern "C" fn __sync_lock_test_and_set_4() {  }

#[no_mangle]
pub extern "C" fn __sync_val_compare_and_swap_2() {  }

#[no_mangle]
pub extern "C" fn __sync_val_compare_and_swap_4() {  }

#[no_mangle]
pub extern "C" fn __sync_synchronize() {  }

#[no_mangle]
pub extern fn _Unwind_Resume() { loop {  } }

// Kernel symbols
#[no_mangle]
pub extern "C" fn initramfs_main() {  }
