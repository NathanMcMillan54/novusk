TARGET = ../../targets/x86_64-novusk.json

all:
	@ echo "Compiling x86_64 kernel..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)
	@ echo "Adding kernel bin to NKUEFI..."
	@ novus_boot_gen_image ../../drivers/boot/nkuefi/target/x86_64-uefi/release/nkuefi.efi target/x86_64-novusk/release/x86_64_kernel x86
