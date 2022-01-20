all: arm
	@ echo "Compiling Aarch64 kernel..."
	@ cargo build --release -Z build-std=core,alloc --target ../../targets/aarch64-novusk.json
	@ echo "Linking..."
	@ aarch64-linux-gnu-ld ../arm/src/boot/start/a53_start.o target/aarch64-novusk/release/libaarch64_kernel_lib.a ../arm/target/aarch64-novusk/release/libarm_kernel_lib.a -o src/boot/kernel7.img

arm: boot
	@ echo "Compiling parts of ARM kernel..."
	@ $(MAKE) -C ../arm TARGET=../../targets/aarch64-novusk.json

boot:
	@ aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c ../arm/src/boot/start/a53_start.S -o ../arm/src/boot/start/a53_start.o

clean:
	@ cargo clean && cd ../arm && cargo clean
