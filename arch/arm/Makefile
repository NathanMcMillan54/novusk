TARGET =?

all:
	@ echo "Compiling arm kernel..."
	@ cargo build --release -Z build-std=core,alloc --target $(TARGET)
