all:
	cargo bootimage --target=x86_64-example.json

run: all
	qemu-system-x86_64 ../target/x86_64-example/debug/bootimage-example.bin
