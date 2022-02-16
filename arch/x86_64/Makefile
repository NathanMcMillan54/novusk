KERNEL =?
PLATFORM =?
TARGET = ../../targets/x86_64-novusk.json

ifeq ($(KERNEL), Image)
	KERNEL := xuImage
endif

all: kernel

kernel:
	@ echo "Compiling x86_64 kernel..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)

clean:
	@ cargo clean
