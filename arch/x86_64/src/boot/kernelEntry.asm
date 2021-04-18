;
; Copied and editted from https://github.com/phil-opp/blog_os/blob/first_edition_post_4/src/arch/x86_64/boot.asm
;
; By: Philipp Oppermann
; From: https://os.phil-opp/eddition-1
;

global long_mode_start
extern main

section .text
bits 64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    extern main
    call main

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt
