BIT64 = False
BOOT_FILE =?
CPU =?
KERNEL =
KERNEL_IMG =?
LINKER = src/boot/start/cm_linker.ld
PLATFORM =?
SOC =?
TARGET =?

ifeq ($(BIT64), False)
	TARGET = ../../targets/thumbv7m-novusk.json
endif

ifeq ($(KERNEL), Image)
	KERNEL_IMG := adImage
endif

ifeq ($(PLATFORM), stellaris6965)
	CPU = cortex-m4
	BOOT_FILE = src/boot/start/cm_boot
else ifeq ($(PLATFORM), rpi2b)
	CPU = cortex-a7
	BOOT_FILE = src/boot/start/a7_start
endif

all: boot kernel link

boot:
	@ arm-linux-gnueabi-gcc -mcpu=$(CPU) -c $(BOOT_FILE).S -o $(BOOT_FILE).o

kernel:
	@ echo "Compiling arm kernel..."
	@ cargo build --lib --release --features $(PLATFORM),$(SOC) -Z build-std=core,alloc --target $(TARGET)

link:
	@ cp -r src/boot/start/$(PLATFORM)_mem.x src/boot/start/memory.x
	@ arm-linux-gnueabi-ld $(BOOT_FILE).o target/thumbv7m-novusk/release/libarm_kernel_lib.a ../../include/novusk/kms/device.km -T$(LINKER)  -o src/boot/$(KERNEL_IMG)

clean:
	@ cargo clean
	@ rm -rf src/boot/adImage
	@ rm -rf src/boot/*.o
	@ rm -rf src/boot/start/*.o
	@ rm -rf src/boot/start/memory.x
	@ rm -rf src/include/dif/kernel_dif.dif
