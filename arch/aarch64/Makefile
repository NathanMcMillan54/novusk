KERNEL =
KERNEL_IMG =?
KMS = ../../include/novusk/kms/
KMS_LIST = $(KMS)device.km $(KMS)display.km
PLATFORM =?

ifeq ($(KERNEL), Image)
	KERNEL_IMG = aadImage
endif

all: arm boot
	@ echo "Compiling Aarch64 kernel..."
	@ cargo build --release -Z build-std=core,alloc --features $(PLATFORM) --target ../../targets/aarch64-novusk.json
	@ echo "Linking..."
	@ aarch64-linux-gnu-ld -Tsrc/boot/linker.ld ../arm/src/boot/start/a53_start.o target/aarch64-novusk/release/libaarch64_kernel_lib.a $(KMS_LIST) -o src/boot/$(KERNEL_IMG)
	@ echo "Created kernel image:"
	@ echo "arch/aarch64/src/boot/$(KERNEL_IMG)"

arm:
	@ echo "Compiling parts of ARM kernel..."
	@ $(MAKE) -C ../arm kernel BITS64=True TARGET=aarch64-novusk PLATFORM=$(PLATFORM)

boot:
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c ../arm/src/boot/start/a53_start.S -o ../arm/src/boot/start/a53_start.o

clean:
	@ cargo clean
	@ cd ../arm && make clean
	@ rm -rf src/boot/aadImage
