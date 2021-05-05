# Running aarch64 Novusk

Running aarch64 Novusk depends on how it was compiled.


### Physical Hardware
If you compiled Novusk for the UEFI aarch64 target, you can just put the ``Image`` file on a USB/SD formatted for FAT32 
then just put it on a board or computer that supports UEFI/EFI. 

Running aarch64 Novusk without UEFI is different depending on what board you're using. Use the ``objcopy`` command for
aarch64/arm32 (don't know yet) to make the kernel image executable on a board. To get the image on a real board, you
should read the board's documentation to see how it is done.

### Simulating in Qemu
Qemu doesn't support UEFI so the command to run it is going to be very different than what is usually used, right now
there is no Novusk documentation explaining how to run aarch64 Novusk in Qemu.

Running Novusk without UEFI in Qemu can be done with this main command:
```commandline
qemu-system-aarch64 -machine <board> -m <memory> -cpu <CPU> -<graphic> -kernel src/boot/Image
```

Virt board example:
```commandline
qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a57 -nographic -kernel src/boot/Image
```
