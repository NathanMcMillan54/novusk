ARCH = x86_64 # For now
TARGET = ../../../targets/$(ARCH)-ueif.json

all:
	@ echo "Compiling Novusk Kernel UEFI..."
	@ cargo build --release -Z build-std=core,alloc --target ../../../targets/$(ARCH)-uefi.json
	@ novus_boot_gen_image target/$(ARCH)-uefi/release/nkuefi.efi x86

clean:
	@ cargo clean
	@ rm -rf *.efi
	@ rm -rf *.img
