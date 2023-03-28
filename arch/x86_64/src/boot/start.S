.intel_syntax noprefix

.code64

.globl start_main
.globl die
.extern main

start_main:
    call clear_regs
    call main

    call die

clear_regs:
    cli
    xor ax, ax
    mov ax, ss
    mov ax, ds
    mov ax, es
    mov ax, fs
    mov ax, gs
    xor eax, eax
    xor ebx, ebx
    xor ecx, ecx
    xor edx, edx
    xor esi, esi
    xor edi, edi
    cld

    ret

die:
    cli
    hlt
    jmp die
