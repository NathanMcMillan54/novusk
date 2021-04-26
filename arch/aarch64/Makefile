BOARD =?

kernel:
	@ cd ../../ && cargo build --target=targets/aarch64-novusk.json

boot:
	@ mv ../../target/aarch64-novusk/debug/novusk src/boot/zImage
