# Virt Qemu board

The Virt machine is a virtual board option that isn’t real (it’s only for Qemu). The Virt board drivers in 
``arch/aarch64/`` is supposed to be used as a template for adding new aarch64 boards.

### How to use the Virt template?

Almost all microcontrollers, microcomputers, boards, etc... have lots of things in common. The Virt driver has 
everything most aarch64 boards need.

To support a new board you need to add:
-  A linker script with the board name (if needed)
- A target file - read ``compiling/targets.md``
- A start file - calls board setup then kernel setup
- An info file - constants describing the board and kernel
- A uart driver - Just need UART0

Read the ``virt/`` directory in ``arch/aarch64/drivers/`` to get an idea on how to write kernel support for a new board.
