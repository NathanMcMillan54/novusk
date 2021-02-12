#!/usr/bin/python3
import os


def main():
    print("Creating x86/x86_64 Novusk image...")
    os.system('cargo bootimage --target arch/x86/src/configs/x86-novusk.json')
    os.system('cp -r target/x86-novusk/debug/bootimage-novusk.bin novusk')
    print("Created image for Novusk")


if __name__ == '__main__':
    main()
