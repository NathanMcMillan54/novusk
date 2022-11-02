.section .text.semihost_syscall
.globl semihost_syscall

    .thumb_func

semihost_syscall:
    bkpt 0xAB
    bx lr
