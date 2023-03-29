.code64

.globl CPU_BRAND
.globl check_cpuid
.globl set_CPU_BRAND

    CPU_BRAND:
        .ascii ""
        .byte 0

set_CPU_BRAND:
    xor eax, eax
    cpuid

    ret

check_cpuid:
    mov $0x80000000, %eax
    cpuid
    cmp $0x80000004, %eax
    jl check_cpu_err

    ret

check_cpu_err:
    mov $1, %rax
    ret
