all:
	@ cargo build --release -Z build-std=core,alloc --target ../../../targets/thumbv7em-none-eabihf.json
	@ mv target/thumbv7em-none-eabihf/release/libstm.a ../../../include/novusk/kms/device.km
