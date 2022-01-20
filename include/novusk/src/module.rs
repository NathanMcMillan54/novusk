#[macro_export]
macro_rules! module_init {
    ($km_init:ident, $km_init_fun:ident) => {
        #[no_mangle]
        pub extern "C" fn $km_init() {
            unsafe { $km_init_fun(); }
        }
    };
}

#[macro_export]
macro_rules! module_end {
    ($km_end:ident, $km_end_fun:ident) => {
        #[no_mangle]
        pub extern "C" fn $km_end() {
            unsafe { $km_end_fun(); }
        }
    };
}

#[macro_export]
macro_rules! start_module {
    ($km_init:ident, $km_end:ident) => {
        extern "C" {
            fn $km_init();
            fn $km_end();
        }

        unsafe {
            $km_init();
            $km_end();
        }
    };
}
