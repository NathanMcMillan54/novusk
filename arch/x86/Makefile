ARCH =?
TARGET_ARCH =?
BOARD =?

all:
	@ echo "Compiling x86 Novusk"
	@ cargo build --target targets/$(TARGET_ARCH)-novusk.json --features $(BOARD)_machine # -Z build-std=core -Z build-std-features=compiler-builtins-mem

image:
	@ echo "Creating kernel images..."
	@ ./disk_img target/$(TARGET_ARCH)-novusk/debug/x86.efi
	@ cp -r target/$(TARGET_ARCH)-novusk/debug/x86.fat src/boot/bzImage
	@ echo "Created bzImage at:"
	@ echo "arch/x86/src/boot/bzImage"

clean:
	@ cargo clean
	@ rm -rf disk_img
	@ rm -rf src/boot/bzImage
