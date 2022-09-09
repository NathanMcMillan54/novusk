HOST =?
TARGET =?
BUILD_ARGS = --release --target ../../targets/$(TARGET).json # FIXME: Use novusk targets

default: clean libs headers mv_files
	@ # mv target/aarch64-apple-darwin/release/*.a lib/ # FIXME same as above

libs:
	@ echo "Compiling libc..."
	cargo rustc --crate-type=staticlib -p stdint_intn $(BUILD_ARGS)
	cargo rustc --crate-type=staticlib -p stdint_uintn $(BUILD_ARGS)
	cargo rustc --crate-type=staticlib -p stddef $(BUILD_ARGS)
	cargo rustc --crate-type=staticlib -p stdint $(BUILD_ARGS)
	cargo rustc --crate-type=staticlib -p stdlib $(BUILD_ARGS)
	cargo rustc --crate-type=staticlib -p string $(BUILD_ARGS)

headers:
	@ echo "Generating header files... "
	cargo +stable run -p genheaders --release --target $(HOST)

mv_files:
	@ echo "Moving files..."
	@ mkdir -p inc
	@ mkdir -p inc/bits
	@ mkdir -p lib
	mv *.h inc/
	mv bits/*.h inc/bits/
	mv target/$(TARGET)/release/*.a lib/
	mv lib/libstdint_intn.a lib/libstdint-intn.a
	mv lib/libstdint_uintn.a lib/libstdint-uintn.a

clean:
	@ cargo clean
	@ rm -rf inc/* lib/* test/build/*
