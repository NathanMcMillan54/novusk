.globl shutdown

shutdown:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0

.equ PSCI_SYSTEM_OFF, 0x84000008
