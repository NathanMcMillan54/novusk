ARCH =?
TARGET = ../../../targets/$(ARCH)-ueif.json

all:
	@ echo "Compiling Novusk Kernel UEFI..."
	@ cargo build --target ../../../targets/$(ARCH)-uefi.json
	@ novus_boot_gen_image target/x86_64-uefi/debug/nkuefi.efi x86
