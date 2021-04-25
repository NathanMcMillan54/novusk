## Running x86_64 Novusk

### Warning:
x86_64 Novusk hasn't been test on a real machine so if you <b><u>REALLY</u></b> want to help or just have a computer 
you're not using, go ahead and try to run the kernel or an OS on it.


---

This will explain how to run Novusk in Qemu which is used for simulating hardware so you can run software that your 
computer can't run.

After you've compiled Novusk, look for the file ``bzImage`` in ``arch/x86_64/src/boot/`` and run the command:

```commandline
qemu-system-x86_64 bzImage
```

This will show a warning, to get rid of that run:

```commandline
qemu-system-x86_64 -drive format=raw,file=bzImage
```

Qemu will launch a window a display the output of Novusk.
