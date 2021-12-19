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
    ($syscall_name:ident, $sys_name:ident, $syscall_num:expr, $syscall_fun:ident) => {
        #[no_mangle]
        pub static $syscall_name: usize = $syscall_num;

        #[no_mangle]
        pub extern "C" fn $sys_name(sys_arg: u8) {
            $syscall_fun(sys_arg);
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

#[macro_export]
macro_rules! define_graphics_pixel {
    ($pixel_fun:ident) => {
        #[no_mangle]
        pub extern "C" pixel(x: usize, y: usize, color: usize) {
            let pixel: fn(usize, usize, usize) = $pixel_fun;
            pixel();
        }
    };
}
