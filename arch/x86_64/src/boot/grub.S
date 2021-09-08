.extern grub_start
.globl _start
.type _start, @function

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

_start:
	movl $stack, %esp
	call grub_start
    call grub_halt

.grub_halt:
    hlt
    jmp grub_halt

.size _start, . - _start
