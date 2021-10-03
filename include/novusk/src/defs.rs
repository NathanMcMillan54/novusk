// Macros for "defining" important functions

#[macro_export]
macro_rules! define_led_blink_function {
    ($blink:ident) => {
        #[no_mangle]
        pub extern "C" fn led_blink(sleep: usize) {
            $blink(sleep);
        }
    };
}

#[macro_export]
macro_rules! define_elf_starter_function {
    ($elf_starter:ident) => {
        #[no_mangle]
        pub extern "C" fn start_elf_file(file: &str) {
            $elf_starter(file);
        }
    };
}

#[macro_export]
macro_rules! define_syscall {
    ($sys_fun_name:ident, $sys_fun:path) => {
        #[no_mangle]
        pub extern "C" fn $sys_fun_name(sys_arg: u8) -> u8 {
            unsafe {
                let sf: fn(u8) -> u8 = $sys_fun;

                return sf(sys_arg);
            }
        }
    };
}

#[macro_export]
macro_rules! define_ethernet_init {
    ($init:ident) => {
        #[no_mangle]
        pub extern "C" fn ethernet_init() {
            let i: fn() = $init;
            i();
        }
    };
}

#[macro_export]
macro_rules! define_wireless_init {
    ($init:ident) => {
        #[no_mangle]
        pub extern "C" fn wireless_init() {
            let w: fn() = $init;
            w();
        }
    };
}
