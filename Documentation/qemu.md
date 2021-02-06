## QEMU
This file explains how to simulate Novusk

---

After compiling Novusk a file should've been made called 'novusk'. This is the compiled file of Nobusk which should be
able to run on a device. But this will just explain how simulate it in qemu.

Run:
```shell
qemu-system-x86_64 -drive -format=raw,file=novsuk
```

Or you could just run:
```shell
make qemu_x86
```
