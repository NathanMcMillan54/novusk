.globl get_x86_cpuid

get_x86_cpuid:
    mov $0x0, %eax
    cpuid
    mov %ebx, 0
    mov %edx, 0
    mov %ecx, 0
