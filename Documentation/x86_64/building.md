## Building x86_64 Novusk

If you don't think you have some of the requirements
([requirements](https://github.com/NathanMcMillan54/novusk/tree/master/Documentation/x86_64/requirements.md)), you can
install them by running:

```commandline
cd arch/x86_64/
make install
```

This will update Rust and install the default bootloader and some build tools.

To start compiling, go back to the root of the project directory (if you aren't already there), then run these commands:

```commandline
make all ARCH=x86_64
```

This will compile the kernel and make an executable of the kernel that you can run. The executable can be found in 
``arch/x86_64/src/boot/`` the file will be named ``bzImage``.
