all:
	@ echo "Compiling VgaG driver..."
	@ cargo build --release -Z build-std=core,alloc --target ../../../targets/x86_64-novusk.json
	@ mv target/x86_64-novusk/release/libvgag.a ../../../include/novusk/kms/display.km

clean:
	@ cargo clean
