# Novusk - Compiling

Novusk uses ``cargo``, ``gcc``, ``ld``, and ``make`` for building the kernel so their latest versions should be installed. 
The main ``Makefile`` builds a crate called ``buildkern`` which compiles most of the kernel based off the configuration
file, you can read more about kernel configs [here](link). Compiling Novusk is done with three ``make`` commands which 
all need an argument ``ARCH``, it needs to be set to the architecture Novusk is being compiled for.

---

### Setup build:

```commandline
make setup_build ARCH=<arch> DIF=<dif_name>.dif
```

This will compile ``buildkern`` and move the DIF file for the kernel to where it needs to be, if the kernel is being 
compiled for no specific device ``DIF`` doesn't need to be set.

### Build kernel:

```commandline
make buildkern ARCH=<arch> DEFCONFIG<True/False> PLATFORM=<device>
```

Running this ``make`` command runs ``buildkern`` with the configuration file as an argument, [this](link) talks about 
setting kernel configurations.

### Linking the kernel:

```commandline
make link ARCH=<arch>
```

This command is used for linking the kernel to bring all the drivers and libraries together so the kernel can run. This
only needs the ``ARCH`` argument set which should be the same as it was in the previous commands.
