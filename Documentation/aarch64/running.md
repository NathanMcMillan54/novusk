# Running aarch64 Novusk

Running aarch64 Novusk depends on how it was compiled.


### Physical Hardware
If you compiled Novusk for the UEFI aarch64 target, you can just put the ``Image`` file on a USB/SD formatted for FAT32 
then just put it on a board or computer that supports UEFI/EFI. 

### Simulating in Qemu
If you compiled Aarch64 Novusk for UEFI and you want to simulate it in Qemu, you need to install a few files: 
``OVMF_CODE.fd``, and ``OVMF_VARS.fd``. There is documentation on OVMF files from Tianocore, 
[here](https://github.com/tianocore/tianocore.github.io/wiki/OVMF)

This is the main command:
```commandline
qemu-system-aarch64 -machine <machine_name> -cpu <cpu> -smp 4 -m <memory> -drive if=pflash,format=raw,file=OVMF_CODE.fd,readonly=on -drive if=pflash,format=raw,file=OVMF_VARS.fd,readonly=on -drive format=raw,file=fat:rw:src/boot/Image
```

