use crate::printk;

#[macro_export]
macro_rules! module_init {
    ($km_name_init:ident, $km_name_start:ident) => {
        #[no_mangle]
        pub extern "C" fn $km_name_init() {
            $km_name_start();
        }
    };
}

#[macro_export]
macro_rules! module_end {
   ($km_name_end:ident, $km_name_finish:ident) => {
        #[no_mangle]
        pub extern "C" fn $km_name_end() {
            $km_name_finish();
        }
    };
}

#[macro_export]
macro_rules! start_module {
    ($($km_name_init:ident)*, $($km_name_end:ident)*) => {
        extern "C" {
            fn $($km_name_init)*();
            fn $($km_name_end)*();
        }

        $($km_name_init)*();
        $($km_name_end)*();
    };
}

