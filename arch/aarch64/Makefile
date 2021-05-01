ARCH =?
TARGET_ARCH =?

all:
	@ cargo build --target targets/$(TARGET_ARCH)-novusk.json -Z build-std=core -Z build-std-features=compiler-builtins-mem

image:
	@ echo "Creating kernel images..."
	@ ./disk_img target/$(TARGET_ARCH)-novusk/debug/aarch64.efi
	@ cp -r target/$(TARGET_ARCH)-novusk/debug/aarch64.fat src/boot/Image
	@ echo "Created Image at:"
	@ echo "arch/aarch64/src/boot/Image"

clean:
	@ cargo clean
	@ rm -rf disk_img
	@ rm -rf src/boot/Image

