#!/bin/bash

sh tools/grub/build.sh
grub-mkrescue -o tools/grub/boot/novusk.iso tools/grub/
