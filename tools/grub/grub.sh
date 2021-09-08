#!/bin/bash

sh build.sh
grub-mkrescue -o boot/novusk.iso ../grub/
