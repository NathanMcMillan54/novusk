#[macro_export]
macro_rules! module_init {
    ($init:path) => {
        #[export_name = "main"]
        pub unsafe extern "C" fn main() {
            let i: fn () = $init;

            i();
        }
    };
}

#[macro_export]
macro_rules! module_end {
    ($mod_end:path) => {
        #[export_name = "end"]
        pub unsafe extern "C" fn end() {
            let e: fn() = $mod_end;

            e();
        }
    };
}
