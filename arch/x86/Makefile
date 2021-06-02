ARCH =?
TARGET_ARCH =?
BOARD =?

all:
	@ echo "Compiling x86 Novusk"
	@ cargo build --target targets/$(TARGET_ARCH)-novusk.json --features $(BOARD)

image:
	@ echo "Creating kernel images..."
	@ novus_boot target/$(TARGET_ARCH)-novusk/debug/x86.efi
	@ cp -r target/$(TARGET_ARCH)-novusk/debug/x86.fat src/boot/bzImage
	@ echo "Created bzImage at:"
	@ echo "arch/x86/src/boot/bzImage"

clean:
	@ cargo clean
	@ rm -rf disk_img
	@ rm -rf src/boot/bzImage
