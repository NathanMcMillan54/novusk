.globl wait_for_event

wait_for_event:
    wfe
    b wait_for_event
