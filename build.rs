use std::fs::{File, write};

extern crate serde_json;
use serde_json::{from_reader, Value};

fn write_space(file: &str) -> std::io::Result<()> {
    write(file, "\n")
}

const VGA_DRIVER: &str = "arch/src/x86/drivers/vga_text.rs";

fn setup_screen_200() {
    write(VGA_DRIVER, "pub const BUFFER_HEIGHT: usize = 25; pub const BUFFER_WIDTH: usize = 80; pub const SCREEN_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH; pub const VGA_BUFFER: i32 = 0xb8000;");
}

fn setup_screen_6400() {
    write(VGA_DRIVER, "pub const BUFFER_HEIGHT: usize = 200; pub const BUFFER_WIDTH: usize = 320; pub const SCREEN_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH; pub const VGA_BUFFER: i32 = 0xa0000;");
}

fn build_x86() {
    /* ----- Read JSON file ----- */
    let defconfig_file = File::open("arch/src/x86/configs/defconfig.json").unwrap();
    let defconfig: Value = from_reader(defconfig_file)
        .expect("Couldn't read json from defconfig file");
    /* ----- ----- */
    let screen_size = defconfig.get("screen_size")
        .expect("Couldn't read screen_size value from defconfig");
    if screen_size == "200" {
        setup_screen_200();
    } else if screen_size == "6400" {
        setup_screen_6400();
    } else { setup_screen_200(); }
}

fn main() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    build_x86();
}