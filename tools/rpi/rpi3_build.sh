#!/bin/bash

echo "Compiling kernel..."
mkdir exclude/
sh tools/aarch64_build.sh
mv target/aarch64-none-novusk/release/Image exclude/kernel8.img
sh tools/rpi/install.sh
