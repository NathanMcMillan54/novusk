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

// These will likely be used in the future. It will make module defining and calling easier.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModuleType {
    Device,
    DeviceIrq,
    RunDeviceIrqs,
    StorageDevice,
    FileSystem,
    GraphicsDriver,
    Other
}

#[macro_export]
macro_rules! v4_module_init {
    ($module_type:pat, $km_name_init:ident, $km_name_start:ident) => {
        #[export_name = stringify!([< $module_type _init >])]
        #[no_mangle]
        pub extern "C" fn $km_name_init() {
            unsafe { $km_name_start(); }
        }
    };
}
