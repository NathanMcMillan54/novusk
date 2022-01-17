TARGET = ../../targets/x86_64-novusk.json

all: boot kernel
	@ echo "Adding kernel bin to NKUEFI..."
	@ novus_boot_gen_image ../../drivers/boot/nkuefi/target/x86_64-uefi/release/nkuefi.efi target/x86_64-novusk/release/x86_64_kernel x86

boot:
	@ echo "Compiling x86_64 boot files..."
	@ x86_64-linux-gnu-as -o src/boot/header.o src/boot/header.S

kernel:
	@ echo "Compiling x86_64 kernel..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)

