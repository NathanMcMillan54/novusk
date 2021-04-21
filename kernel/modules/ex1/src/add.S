.text

.globl addInt
addInt:
    addq %rdi, %rsi
    movq %rsi, %rax
    ret
