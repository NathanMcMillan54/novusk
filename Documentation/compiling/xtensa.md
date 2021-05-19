# Compiling Xtensa Novusk

Xtensa Novusk is a lot smaller than x86 and aarch64 Novusk so compiling is very different and can take a long time. If
you have a Rust Xtensa compiler you can compile this project normally with the board name added in features.

If you don't have an Xtensa compiler read ``xtensa/install.md``.

To compile Xtensa Novusk, compile it the same way you would with any Xtensa project with a feature ``esp32_board`` so 
you can use the esp32 drivers.
