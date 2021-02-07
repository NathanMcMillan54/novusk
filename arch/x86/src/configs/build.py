#!/usr/bin/python3
import sys


def main(os):
    print("arch/x86/src/configs/build.py")
    print("Compiling for defconfigs with python")
    os_mod = open("drivers/src/os/mod.rs", "r+")
    os_mod.writelines(f"pub mod setup;\npub mod {os};")
    os_setup = open("drivers/src/os/setup.rs", "r+")
    sc = "{"
    ec = "}"
    sq = '"'
    eq = '"'
    setup_function = f"pub fn setup() {sc}\n    {os}::setup();\n{ec}"
    os_setup.writelines(f"use super::{os};\n\n{setup_function}")
    os_name = open("src/userspace/name.rs", "r+")
    os_name.writelines(f"pub static OSNAME: &'static str = {sq}{os}{eq};")


if __name__ == '__main__':
    os_name = sys.argv[1]
    main(os_name)
