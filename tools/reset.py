#!/usr/bin/python3

def reset_aarch64():
    print("Resetting aarch64 settings...")
    uart_driver = open("arch/aarch64/src/drivers/uart.rs", "r+")
    uart_driver.writelines("pub const UART0: *mut u8 = 0x0900_0000 as *mut u8;")


def reset_x86():
    print("Resetting x86...")


def reset_kernel_configs():
    print("Resetting kernel configs...")
    screen_driver = open("arch/x86/src/drivers/vga_text.rs", "r+")
    screen_driver.writelines("pub const BUFFER_HEIGHT: usize = 25; pub const BUFFER_WIDTH: usize = 80; pub const SCREEN_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH; pub const VGA_BUFFER: i32 = 0xb8000;")
    os_mod = open("drivers/src/os/mod.rs", "r+")
    os_mod.writelines(f"pub mod setup;\npub mod none;")
    os_setup = open("drivers/src/os/setup.rs", "r+")
    sc = "{"
    ec = "}"
    sq = '"'
    eq = '"'
    setup_function = f"pub fn setup() {sc}\n    none::setup();\n{ec}"
    os_setup.writelines(f"use super::none;\n\n{setup_function}")
    os_name = open("src/userspace/name.rs", "r+")
    os_name.writelines(f"pub static OSNAME: &'static str = {sq}none{eq};")


def main():
    print("tools/reset.py")
    reset_aarch64()
    reset_x86()
    reset_kernel_configs()


if __name__ == '__main__':
    main()
