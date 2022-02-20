BIT64 = False
KERNEL =
KERNEL_IMG =?
PLATFORM =?
SOC =?
TARGET =?

ifeq ($(BIT64), False)
	TARGET = thumbv7m-none-eabi
endif

ifeq ($(KERNEL), Image)
	KERNEL_IMG := adImage
endif

all: boot kernel link

boot:
	@ arm-linux-gnueabi-gcc -mcpu=cortex-m4 -c src/boot/start/cm_boot32.S -o src/boot/start/cm_boot32.o

kernel:
	@ echo "Compiling arm kernel..."
	@ cargo build --release --features $(PLATFORM),$(SOC) -Z build-std=core,alloc --target $(TARGET)

link:
	@ arm-linux-gnueabi-ld src/boot/start/cm_boot32.o target/thumbv7m-none-eabi/release/arm_kernel_bin ../../include/novusk/kms/device.km -o src/boot/$(KERNEL_IMG)
