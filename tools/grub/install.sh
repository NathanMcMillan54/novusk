#!/bin/bash

curl https://raw.githubusercontent.com/phil-opp/blog_os/first_edition_post_8/src/arch/x86_64/boot.asm > tools/grub/grub_boot.asm
curl https://raw.githubusercontent.com/phil-opp/blog_os/first_edition_post_8/src/arch/x86_64/long_mode_init.asm > tools/grub/long_mode_init.asm
curl https://raw.githubusercontent.com/phil-opp/blog_os/first_edition_post_8/src/arch/x86_64/multiboot_header.asm > tools/grub/multiboot_header.asm
