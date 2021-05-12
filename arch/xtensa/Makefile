BOARD =? esp32
MY_ARCH =? x86_64

all: setargs
	@ cargo build --features $(BOARD)_board
	@ mv target/debug/xtensa src/boot/uImage

setargs:
	@ CUSTOM_RUSTC=tools/rust-xtensa
	@ export RUST_BACKTRACE=1
	@ export XARGO_RUST_SRC=$CUSTOM_RUSTC/library
	@ export RUSTC=$CUSTOM_RUSTC/build/$(ARCH)-unknown-linux-gnu/stage2/bin/rustc
	@ export RUSTDOC=$CUSTOM_RUSTC/build/$(ARCH)-unknown-linux-gnu/stage2/bin/rustdoc
