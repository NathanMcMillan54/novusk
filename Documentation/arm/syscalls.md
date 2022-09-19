# Novusk - ARM Syscalls

System calls allow a user to interact with the kernel, they are useful for using kernel functions and getting system 
information. This page talks about how system calls work and are used in ARM Novusk.


---


### ARM 32

To invoke a system call in ARM 32 Novusk registers ``r3`` - ``r7`` need to be used, the return of the system call is 
stored in ``r8``. The system call number is stored in ``r3`` and the system call's arguments are stored in ``r4`` - 
``r7``. Here is an example of a system call:

```asm
    mov r3, 1 // write system call
    mov r4, 3 // ``option``
    mov r5, 91 // ``write``
    mov r6, 1 // ``len``
    // mov r7, 0 <-- write only has 3 arguments
    svc #0 // Invokes the system call
    
    mov sys_ret, r8 // The system call return
```

---

### ARM 64

---

System calls are associated with numbers so the system call handler can tell what system call is being invoked, all 
common system calls have the same number on different architectures. Here is a list of ARM specific system calls:



---

- [common_syscalls.md](../kernel/common_syscalls.md)
