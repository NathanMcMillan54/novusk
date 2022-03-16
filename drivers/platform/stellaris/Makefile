all:
	@ cargo build --release -Z build-std=core,alloc --target ../../../targets/thumbv7m-none-eabi.json
	@ mv target/thumbv7m-none-eabi/release/libstellaris.a ../../../include/novusk/kms/device.km
