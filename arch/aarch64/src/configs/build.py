#!/usr/bin/python3
import sys


def main(os, board):
    print("arch/aarch64/src/configs/build.py")
    print("Compiling for defconfigs...")
    q = '"'
    boot_name = open("arch/aarch64/src/boot/name.rs", "r+")
    boot_name.writelines(f"pub const BOARDNAME: &'static str = {q}{board}{q};")


if __name__ == '__main__':
    os_name = sys.argv[1]
    board_name = sys.argv[2]
    main(os_name, board_name)
