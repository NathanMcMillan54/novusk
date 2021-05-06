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

---

Some microcomputers can be very complex and different than most aarch64 boards. If you're making support for a 
microcomputer that is complex and different than most boards, you should make a new cargo project and add it to the
``aarch64`` workspace so it's far away from the ``src/`` so you have a clean place to ``_start`` developing (joke). This
workspace extension doesn't need to use the dependencies from the main project.