ARCH =?
TARGET =?

all:
	@ echo "Compiling Novuskinc..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)
	@ cd .
	@ $(ARCH)-linux-gnu-ar -crs libnovuskinc.a target/$(ARCH)-novusk/release/libnovuskinc.rlib build/display.km build/device.km

clean:
	@ cargo clean
	@ rm -rf libnovuskinc.a
	@ rm -rf build/
