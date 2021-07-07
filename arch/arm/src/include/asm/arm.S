.globl wait_fe

wait_fe:
    wfe
    b wait_fe
