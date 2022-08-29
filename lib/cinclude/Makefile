HOST =?
TARGET =?
BUILD_ARGS = --crate-type=staticlib --release --target $(TARGET)

default: clean
	@ echo "Compiling libc..."
	cargo rustc -p stdint_intn $(BUILD_ARGS)
	cargo rustc -p stdint_uintn $(BUILD_ARGS)
	cargo rustc -p stddef $(BUILD_ARGS)
	cargo rustc -p stdint $(BUILD_ARGS)
	cargo rustc -p stdlib $(BUILD_ARGS)
	cargo rustc -p string $(BUILD_ARGS)
	@ echo "Generating header files... "
	cargo run -p cinclude --bin cinclude-bin --release --target $(HOST)
	@ echo "Moving files..."
	mv *.h inc/
	@ mkdir inc/bits
	@ mkdir lib
	mv bits/*.h inc/bits
	mv target/$(TARGET)/release/*.a lib/

clean:
	@ cargo clean
	@ rm -rf inc/* lib/* test/build/*
