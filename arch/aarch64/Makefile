all: arm
	@ echo "Compiling Aarch64 kernel..."
	@ cargo build --release --target ../../targets/aarch64-novusk.json

arm:
	@ echo "Compiling parts of ARM kernel..."
	@ cd ../arm

clean:
	@ cargo clean && cd ../arm && cargo clean
