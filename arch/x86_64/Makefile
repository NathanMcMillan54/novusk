TARGET = ../../targets/x86_64-novusk.json

all: kernel

kernel:
	@ echo "Compiling x86_64 kernel..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)

