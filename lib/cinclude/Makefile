HOST =?
TARGET =?
BUILD_ARGS = --release --target ../../targets/$(TARGET).json

default: clean
	@ echo "Compiling libc..."
	cargo build -p stdint_intn $(BUILD_ARGS)
	cargo build -p stdint_uintn $(BUILD_ARGS)
	cargo build -p stddef $(BUILD_ARGS)
	cargo build -p stdint $(BUILD_ARGS)
	cargo build -p stdlib $(BUILD_ARGS)
	cargo build -p string $(BUILD_ARGS)
	@ echo "Generating header files... "
	cargo run -p cinclude --bin cinclude-bin --release --target $(HOST)
	@ echo "Moving files..."
	mv *.h inc/
	@ mkdir inc/bits
	@ mkdir lib
	mv bits/*.h inc/bits/
	mv target/$(TARGET)/release/*.a lib/

clean:
	@ cargo clean
	@ rm -rf inc/* lib/* test/build/*
