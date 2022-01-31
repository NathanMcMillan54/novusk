use std::env;
use std::ffi::OsString;

/// Recommended cross compiler command
pub fn cc_cmd() -> &'static str {
    let aarch64 = OsString::from("aarch64");
    let arm = OsString::from("arm");
    let x86_64 = OsString::from("x86_64");

    return match env::var_os("CARGO_CFG_TARGET_ARCH").unwrap() {
        aarch64 => "aarch64-linux-gnu",
        arm => "arm-linux-gnueabi",
        x86_64=> "x86_64-linux-gnu",

        _ => panic!("Unknown target"),
    }
}
