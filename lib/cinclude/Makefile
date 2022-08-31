HOST =?
TARGET =?
BUILD_ARGS = --release --target aarch64-apple-darwin # FIXME: Use novusk targets

default: clean
	@ echo "Compiling libc..."
	cargo build -p stdint_intn $(BUILD_ARGS)
	cargo build -p stdint_uintn $(BUILD_ARGS)
	cargo build -p stddef $(BUILD_ARGS)
	cargo build -p stdint $(BUILD_ARGS)
	cargo build -p stdlib $(BUILD_ARGS)
	cargo build -p string $(BUILD_ARGS)
	@ echo "Generating header files... "
	cargo run -p cinclude --bin cinclude-bin --release --target aarch64-apple-darwin # FIXME same as above
	@ echo "Moving files..."
	mv *.h inc/
	@ mkdir -p inc
	@ mkdir -p inc/bits
	@ mkdir -p lib
	mv bits/*.h inc/bits/
	mv target/aarch64-apple-darwin/release/*.a lib/ # FIXME same as above

clean:
	@ cargo clean
	@ rm -rf inc/* lib/* test/build/*
