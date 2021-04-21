#!/bin/bash

BOARD=$1

if [ "$BOARD" = 'esp32' ]; then
    cargo build --target xtensa-esp32-none-elf
else
    echo "$BOARD not supported read https://github.com/new-kernel/new-kernel/#arm-boards for supported boards"
fi
