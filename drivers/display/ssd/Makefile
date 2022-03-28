all:
	@ cargo build --release -Z build-std=core,alloc --target ../../../targets/thumbv7em-none-eabihf.json
	@ rm -rf ../../../include/novusk/kms/display.km

clean:
	@ cargo clean
