.globl wait_for_interrupt

wait_for_interrupt:
    wfi
    j wait_for_interrupt
