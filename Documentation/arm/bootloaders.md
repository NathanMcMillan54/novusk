# Novusk - ARM bootloader documentation

---

There is no specific way to load Novusk on an ARM CPU, you should read the board specific documentation. This will give 
some examples and explanation for using ARM bootloaders for Novusk.


## RPi bootloader:

For almost any RaspberryPi board, you should use the RaspberryPi Imager. Replace ``kernel7.img`` with the Novusk image for 
ARM32, and ``kernel8.img`` for Aarch64.

## STM32Xxxx bootloader:

*Again* there is no specific way of loading ARM Novusk, try to look for ST's documentation on loading and flashing the 
kernel.
