ARCH =?
TARGET =?

all:
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)
	@ cd .
	@ # $(ARCH)-linux-gnu-ar -s -r libnovuskinc.a target/$(ARCH)-novusk/release/libnovuskinc.rlib build/display.km build/device.km

clean:
	@ cargo clean
	@ rm -rf libnovuskinc.a
	@ rm -rf kms/
