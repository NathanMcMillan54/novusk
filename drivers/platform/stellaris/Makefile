all:
	@ cargo build --release -Z build-std=core,alloc --target thumbv7m-none-eabi
	@ mv target/thumbv7m-none-eabi/release/libstellaris.a ../../../include/novusk/kms/device.km
