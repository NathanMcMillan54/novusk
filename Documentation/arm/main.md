# Novusk - ARM Novusk

---

### Compiling

```commandline
make novusk ARCH=<arm version> PLATFORM=<device>
```

``ARCH`` can be set to ``aarch64`` or ``arm`` to set the default targets, you can read more about target information for
ARM in [targets.md](targets.md). [platforms.md](platforms.md) has a list of supported devices for ARM Novusk. If you're 
going to use any features a list of ARM features and there usages can be found in [features.md](features.md).

---

### Capabilities

What the ARM kernel as capable of doing is based on how much support there is for the device it's running on. 
Generally the ARM kernel can: setup early CPU functions, handle memory allocation, handle IRQs and exceptions, 
and handle [system calls](syscall link), and most devices have mailbox support (firmware interface).
