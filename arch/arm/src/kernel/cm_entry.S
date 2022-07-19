.extern arm32_syscall_handler
.globl cortex_m_svcall

cortex_m_svcall:
    tst lr, #4
    ite eq
    mrseq r0, msp
    mrsne r0, psp
    b arm32_syscall_handler

