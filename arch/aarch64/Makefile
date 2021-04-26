BOARD =?

kernel:
	@ cd ../../ && cargo build --target=targets/aarch64-novusk.json

boot:
	@ mv ../../target/aarch64-novusk/debug/novusk_kernel src/boot/zImage

clean:
	@ rm -rf src/boot/zImage
