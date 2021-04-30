ARCH =?
TARGET_ARCH =?

all:
	@ echo "Compiling x86 Novusk"
	@ cargo build --target targets/$(TARGET_ARCH)-novusk.json -Z build-std=core -Z build-std-features=compiler-builtins-mem

image:
	@ echo "Creating kernel images..."
	@ cargo run --package disk_img -- target/$(TARGET_ARCH)-novusk/debug/x86.efi
	@ cp -r target/$(TARGET_ARCH)-novusk/debug/x86.fat src/boot/bzImage
	@ echo "Created bzImage at:"
	@ echo "arch/x86/src/boot/bzImage"

clean:
	@ cargo clean
	@ rm -rf src/boot/bzImage
