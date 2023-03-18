.set ALIGN, 1<<0
.set MEMINFO, 1<<1
.set FLAGS, ALIGN | MEMINFO
.set MAGIC, 0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .bootstrap_stack, "aw", @nobits
stack_bottom:
.skip 16384
stack_top:

.section .text

.extern die
.extern check_cpuid
.extern start_main

.global _start
.type _start, @function

_start:
    movl $stack_top, %esp
    movl %ebx, %edi

    call check_cpuid
    call check_multiboot
    call start_main

    cli
    call die

check_multiboot:
    cmp $0x36d76289, %eax
    mov $0, %al
    ret

.size _start, . - _start
