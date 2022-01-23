ARCH =?
TARGET =?

all:
	@ echo "Compiling Novuskinc..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)
	@ $(ARCH)-linux-gnu-gcc -shared target/$(ARCH)-novusk/release/libnovuskinc.rlib build/display.km build/device.km -static -o libnovuskinc

clean:
	@ cargo clean
	@ rm -rf libnovusk.a
	@ rm -rf build/
