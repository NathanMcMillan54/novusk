// This file might get delteted soon
// TODO: Write documentation on how to boot with grub
.set ALIGN,    1<<0
.set MEMINFO,  1<<1
.set FLAGS,    ALIGN | MEMINFO
.set MAGIC,    0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .bootstrap_stack, "aw", @nobits
stack_bottom:
.skip 16384
stack:

.section .text
.globl grub_hang
.globl _grub_start
.type _grub_start, @function

_grub_start:
	movl $stack, %esp
	call main

.size _start, . - _start
