BIT64 = False
BOOT_FILE =?
CPU =?
KERNEL =
KERNEL_IMG =?
LINKER = src/boot/start/cm_linker.ld
PLATFORM =?
SOC =?
TARGET =?

ifeq ($(KERNEL), Image)
	KERNEL_IMG := adImage
endif

ifeq ($(PLATFORM), stellaris6965)
	CPU = cortex-m3
	BOOT_FILE = src/boot/start/cm_boot
	TARGET = thumbv7m-none-eabi
else ifeq ($(PLATFORM), stm32f407)
	CPU = cortex-m4
	BOOT_FILE = src/boot/start/cm_boot
	TARGET = thumbv7em-none-eabihf
else ifeq ($(PLATFORM), rpi2b)
	CPU = cortex-a7
	BOOT_FILE = src/boot/start/a7_start
	TARGET = arm-a-novusk
endif

boot:
	@ arm-linux-gnueabi-gcc -mcpu=$(CPU) -c $(BOOT_FILE).S -o $(BOOT_FILE).o

link: boot
	@ arm-linux-gnueabi-ld $(BOOT_FILE).o -T$(LINKER) ../../target/$(TARGET)/release/*.a -o src/boot/$(KERNEL_IMG)

clean:
	@ cargo clean
	@ rm -rf src/boot/adImage
	@ rm -rf src/boot/*.o
	@ rm -rf src/boot/start/*.o
	@ rm -rf src/boot/start/memory.x
	@ rm -rf src/include/dif/kernel_dif.dif
