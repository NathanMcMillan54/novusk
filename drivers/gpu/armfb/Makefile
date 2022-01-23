all:
	@ cargo build --release -Z build-std=core,alloc --target ../../../targets/aarch64-novusk.json
	@ mv target/aarch64-novusk/release/libarmfb.a ../../../include/novusk/build/display.km
