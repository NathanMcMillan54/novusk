# PS2 input

The UEFI is a new thing and PS2 devices are pretty old. Most x86 PC mother boards are made with PS2 ports, so it should
work with a CPU that supports UEFI. In future versions of Novusk there will be USB support so keyboard and mouse input 
will definitely work and can run on devices like laptops.

### Files:

### Drivers

x86_64:
- ``arch/x86/src/drivers/ps2_keyboard/``
- ``arch/x86/src/drivers/ps2_mouse/``

### Initialization

- ``drivers/input/``
