# Compiling Xtensa Novusk

Xtensa Novusk is a lot smaller than x86 and aarch64 Novusk so compiling is very different and can take a long time. If
you have a Rust Xtensa compiler you can compile this project normally with the board name added in features.

If you don't have an Xtensa compiler read ``xtensa/install.md``.

To compile Xtensa Novusk, run main command:
```commandline
cd arch/xtensa/
make all BOARD=<board_name> MY_ARCH=<your_architecture>
```

Example:
```commandline
make all BOARD=esp32 MY_ARCH=x86_64
```

``MY_ARCH`` needs to be set to your CPU architecture, read ``xtensa/support.md`` for a list of supported boards.