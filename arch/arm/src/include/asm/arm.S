.globl wait_for_event
.globl no_op

wait_for_event:
    wfe
    b wait_for_event

no_op:
    nop
