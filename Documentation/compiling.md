## Compiling Novusk
This file explains how to compile Novusk

---

Main command:
```shell
make all
```

This is the main command that compiles Novusk.

Arguments:
```shell
ARCH=<architecture>
OS=<os name>
```

The ARCH argument is for specifying what architecture Novusk will run on. As of v1.0.0 The only supported architecture
is x86/x86_64 so you'll have to give it the argument ``x86``. The OS argument is the name of the OS Novusk will run, if
you don't want Novusk to run an OS give it the argument ``none``.

Example command:
```shell
make all ARCH=x86 OS=none
```