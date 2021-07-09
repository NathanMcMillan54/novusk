.text

.globl add

add:
    addq %rdi, %rsi
    movq %rsi, %rax
    ret
