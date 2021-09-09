#!/bin/bash

sh tools/grub/build.sh
grub-mkrescue -o boot/novusk.iso ../grub/
