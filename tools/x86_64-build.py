#!/bin/python3
import os
import sys

def main():
    args = sys.argv
    boot = args[1]
    bootloader = args[2]

    if bootloader == "grub":
        print(f"Build x86_64 Novusk for {bootloader} with {boot}")
        os.system(f"cargo build -p x86_64@0.1.0 --features {boot},{bootloader} --target targets/x86_64-novusk.json")
    elif bootloader == "bootloader_rs":
        bootloader_version = args[3]
        print("Changing \"bootloader\" version")
        cargo_file = open("arch/x86_64/Cargo.toml")
        file_contents = cargo_file.read()
        file_contents = file_contents.replace('bootloader = "0.9.23"', f'bootloader = "{bootloader_version}"')
        open("arch/x86_64/Cargo.toml", "w+").write(file_contents)
        os.system(f"cargo bootimage -p x86_64@0.1.0 --features {boot},{bootloader}_{bootloader_version.replace('.', '_')} --target targets/x86_64-novusk.json")

if __name__ == '__main__':
    main()
