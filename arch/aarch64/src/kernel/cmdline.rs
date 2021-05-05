use uefi::proto::console::text::Output;

#[no_mangle]
pub unsafe extern "C" fn cmdline_init(stdout: &mut Output) {
    stdout.clear().unwrap().unwrap();
}
